// dial9 Demo - Runtime Telemetry for Tokio
//
// This demo showcases dial9's ability to capture detailed runtime events
// including task polls, wakes, parks, and kernel scheduling delays.
//
// To run this demo:
//   cargo run --bin dial9_demo
//
// Then open the generated trace file in the dial9 trace viewer:
//   https://dial9.dev/viewer (or the official viewer)
//
// The trace files will be saved to the system temp directory as:
//   dial9_demo/trace.0.bin, trace.1.bin, etc.

use std::{path::Path, time::Duration};

use dial9_tokio_telemetry::telemetry::{RotatingWriter, TelemetryHandle, TracedRuntime};
use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinSet,
};

// Constants for trace file rotation settings
const ROTATE_SIZE_BYTES: u64 = 20 * 1024 * 1024; // 20 MiB
const MAX_TOTAL_SIZE_BYTES: u64 = 100 * 1024 * 1024; // 100 MiB

fn main() -> std::io::Result<()> {
    // Get system temp directory for portable paths
    let temp_dir = std::env::temp_dir();
    let trace_dir = temp_dir.join("dial9_demo");
    let trace_base = trace_dir.join("trace.bin");

    // Set up the rotating writer for trace output
    let writer = RotatingWriter::new(
        trace_base.to_str().unwrap_or("/tmp/dial9_demo/trace.bin"),
        ROTATE_SIZE_BYTES,
        MAX_TOTAL_SIZE_BYTES,
    )?;

    // Build a multi-threaded Tokio runtime with tracing enabled
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.worker_threads(4).enable_all();

    let (runtime, guard) = TracedRuntime::build_and_start(builder, writer)?;
    let handle = guard.handle();

    runtime.block_on(async {
        println!("🚀 dial9 demo starting...");
        println!("Trace output directory: {}", trace_dir.display());
        println!("Trace files: trace.0.bin, trace.1.bin, etc.");
        println!();

        // Run various scenarios to generate interesting traces
        run_file_io_demo(&temp_dir).await?;
        run_concurrent_tasks_demo().await?;
        run_tcp_listener_demo().await?;
        run_chained_wakes_demo(&handle).await?;

        println!();
        println!("✅ dial9 demo completed!");
        println!("Open the trace files in the dial9 viewer to see the results.");

        Ok(())
    })
}

/// Demo 1: File I/O operations
/// This will show how async file operations interact with the runtime
async fn run_file_io_demo(temp_dir: &Path) -> io::Result<()> {
    println!("📁 Running File I/O demo...");

    let mut tasks = JoinSet::new();

    for i in 0..5 {
        let dir = temp_dir.to_path_buf();
        tasks.spawn(async move {
            let filename = dir.join(format!("dial9_demo_test_{}.txt", i));

            // Write file
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&filename)
                .await?;

            let content = format!("Hello from task {} at {:?}", i, std::time::Instant::now());
            file.write_all(content.as_bytes()).await?;
            file.sync_all().await?;
            drop(file);

            // Read file back
            let mut file = File::open(&filename).await?;
            let mut buf = String::new();
            file.read_to_string(&mut buf).await?;

            tokio::time::sleep(Duration::from_millis(10)).await;

            io::Result::Ok((i, buf))
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok((i, content))) => println!("  Task {} completed: {} bytes", i, content.len()),
            Ok(Err(e)) => println!("  Task error: {}", e),
            Err(e) => println!("  Join error: {}", e),
        }
    }

    Ok(())
}

/// Demo 2: Concurrent tasks with varying workloads
/// This will show task scheduling and work-stealing behavior
async fn run_concurrent_tasks_demo() -> io::Result<()> {
    println!("⚡ Running Concurrent Tasks demo...");

    let mut tasks = JoinSet::new();

    // Spawn tasks with different sleep durations to create scheduling patterns
    for i in 0..10 {
        let sleep_ms = (i + 1) * 10;
        tasks.spawn(async move {
            let start = tokio::time::Instant::now();

            // Simulate work
            tokio::time::sleep(Duration::from_millis(sleep_ms as u64)).await;

            // Small computation to show poll duration
            let mut sum = 0u64;
            for j in 0..10000 {
                sum = sum.wrapping_add(j);
            }

            tokio::time::sleep(Duration::from_millis(5)).await;

            let elapsed = start.elapsed();
            (i, elapsed, sum)
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok((i, elapsed, _)) => println!("  Task {} completed in {:?}", i, elapsed),
            Err(e) => println!("  Join error: {}", e),
        }
    }

    Ok(())
}

/// Demo 3: TCP listener with connections
/// This demonstrates I/O driver interaction and wake events
async fn run_tcp_listener_demo() -> io::Result<()> {
    println!("🌐 Running TCP Listener demo...");

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    println!("  Listening on {}", local_addr);

    // Spawn the acceptor task
    let acceptor = tokio::spawn(async move {
        let mut count = 0;
        let timeout = tokio::time::timeout(Duration::from_millis(500), async {
            loop {
                match listener.accept().await {
                    Ok((mut socket, _addr)) => {
                        count += 1;
                        // Spawn a task to handle the connection
                        tokio::spawn(async move {
                            let mut buf = [0u8; 1024];
                            match socket.read(&mut buf).await {
                                Ok(n) => {
                                    let _ = socket.write_all(&buf[..n]).await;
                                }
                                Err(e) => println!("    Read error: {}", e),
                            }
                        });
                    }
                    Err(e) => {
                        println!("  Accept error: {}", e);
                        break;
                    }
                }
            }
        });

        let _ = timeout.await;
        count
    });

    // Spawn client tasks
    let mut clients = JoinSet::new();
    for i in 0..5 {
        let addr = local_addr;
        clients.spawn(async move {
            tokio::time::sleep(Duration::from_millis(i * 20)).await;

            match tokio::net::TcpStream::connect(addr).await {
                Ok(mut stream) => {
                    let msg = format!("Hello from client {}", i);
                    if let Err(e) = stream.write_all(msg.as_bytes()).await {
                        return (i, Err(e));
                    }

                    let mut buf = [0u8; 1024];
                    match stream.read(&mut buf).await {
                        Ok(n) => {
                            let response = String::from_utf8_lossy(&buf[..n]);
                            (i, Ok(response.to_string()))
                        }
                        Err(e) => (i, Err(e)),
                    }
                }
                Err(e) => (i, Err(e)),
            }
        });
    }

    // Wait for clients to complete
    while let Some(result) = clients.join_next().await {
        match result {
            Ok((i, Ok(response))) => println!("  Client {} received: {}", i, response),
            Ok((i, Err(e))) => println!("  Client {} error: {}", i, e),
            Err(e) => println!("  Client join error: {}", e),
        }
    }

    // Wait for acceptor
    match acceptor.await {
        Ok(count) => println!("  Acceptor handled {} connections", count),
        Err(e) => println!("  Acceptor error: {}", e),
    }

    Ok(())
}

/// Demo 4: Chained wakes
/// This demonstrates how tasks wake each other and move between workers
/// Uses TelemetryHandle::spawn to capture wake-event traces
async fn run_chained_wakes_demo(handle: &TelemetryHandle) -> io::Result<()> {
    println!("🔗 Running Chained Wakes demo...");

    let (tx2, rx2) = tokio::sync::oneshot::channel::<String>();
    let (tx3, rx3) = tokio::sync::oneshot::channel::<String>();

    // Task 3 waits on task 2, which waits on task 1
    // Use handle.spawn() to capture wake edges in dial9 traces
    let task3 = handle.spawn(async move {
        let start = tokio::time::Instant::now();
        match rx3.await {
            Ok(msg) => {
                println!(
                    "  Task 3 received: {} (elapsed: {:?})",
                    msg,
                    start.elapsed()
                );
            }
            Err(_) => println!("  Task 3: channel closed without message"),
        }
        "Task 3 done"
    });

    let task2 = handle.spawn(async move {
        let start = tokio::time::Instant::now();
        match rx2.await {
            Ok(msg) => {
                tokio::time::sleep(Duration::from_millis(20)).await;
                let _ = tx3.send(format!("{} -> Task 2", msg));
                println!(
                    "  Task 2 forwarded message (elapsed: {:?})",
                    start.elapsed()
                );
            }
            Err(_) => println!("  Task 2: channel closed without message"),
        }
        "Task 2 done"
    });

    let task1 = handle.spawn(async move {
        let start = tokio::time::Instant::now();
        tokio::time::sleep(Duration::from_millis(30)).await;
        let _ = tx2.send("Hello from Task 1".to_string());
        println!("  Task 1 sent message (elapsed: {:?})", start.elapsed());
        "Task 1 done"
    });

    // Also demonstrate mpsc channel usage
    let (mpsc_tx, mut mpsc_rx) = tokio::sync::mpsc::channel::<i32>(10);

    let producer = handle.spawn(async move {
        for i in 0..5 {
            tokio::time::sleep(Duration::from_millis(15)).await;
            if mpsc_tx.send(i).await.is_err() {
                break;
            }
        }
    });

    let consumer = handle.spawn(async move {
        let mut sum = 0;
        while let Some(val) = mpsc_rx.recv().await {
            sum += val;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        println!("  Consumer sum: {}", sum);
        sum
    });

    // Wait for all tasks and handle errors properly
    match tokio::try_join!(task1, task2, task3, producer, consumer) {
        Ok((r1, r2, r3, _, sum)) => {
            println!(
                "  All tasks completed: {:?}, {:?}, {:?}, sum={}",
                r1, r2, r3, sum
            );
        }
        Err(e) => {
            eprintln!("  Task failed: {}", e);
        }
    }

    Ok(())
}

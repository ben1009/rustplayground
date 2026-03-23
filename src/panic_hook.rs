// PanicInfo was renamed to PanicHookInfo in Rust 1.82
// For compatibility with both 1.80 (MSRV) and newer versions, we use
// PanicInfo with deprecated allowed since it's the stable API name
#[allow(deprecated)]
use std::panic::PanicInfo;

use backtrace::Backtrace;
use color_backtrace::BacktracePrinter;

pub fn set_panic_hook() {
    // Set a panic hook that records the panic as a `tracing` event at the
    // `ERROR` verbosity level.
    std::panic::set_hook(Box::new(|panic| {
        log_panic(panic);
    }));
}

#[allow(deprecated)]
pub fn log_panic(panic: &PanicInfo<'_>) {
    let backtrace = Backtrace::new();
    let printer = BacktracePrinter::new().lib_verbosity(color_backtrace::Verbosity::Full);
    let colored = printer
        .format_trace_to_string(&backtrace)
        .unwrap_or_default();

    eprintln!("{}", panic);
    eprintln!("{}", colored);
}

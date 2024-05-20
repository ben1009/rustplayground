use std::{fs, io::Write};

use anyhow::{anyhow, Context, Ok, Result};

pub fn crate_files(path: impl AsRef<std::path::Path>, content: &[u8]) -> Result<()> {
    let _ = fs::remove_dir_all(path.as_ref()).context("Failed to remove directory");
    fs::create_dir_all(path.as_ref()).context("Failed to create directory")?;

    for _i in 0..10 {
        let path = path.as_ref().join(format!("{}.log", _i));
        let mut f = fs::File::create(path).context("Failed to create file")?;
        let mut buffer = Vec::new();
        buffer.extend_from_slice(content);
        buffer.extend_from_slice(b"\n");
        f.write_all(&buffer)
            .map_err(|e| anyhow!("Failed to write file: {}", e))?
    }

    Ok(())
}

pub fn read_files(path: impl AsRef<std::path::Path>) -> Result<String> {
    let entries = fs::read_dir(path.as_ref())?.collect::<Vec<_>>();
    let mut ret = String::new();
    for e in entries {
        let s = fs::read_to_string(e?.path()).map_err(|e| anyhow!("Failed to read file: {}", e))?;
        ret.push_str(s.as_str());
    }

    Ok(ret)
}

use std::{
    io,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

pub fn output(program: &str, args: &[&str], timeout: Duration) -> io::Result<String> {
    // A file keeps a verbose child from blocking on a full pipe.
    let path = std::env::temp_dir().join(format!(
        "whylag-{}-{:?}",
        std::process::id(),
        thread::current().id()
    ));
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)?;
    std::fs::remove_file(&path)?;
    let mut child = Command::new(program)
        .args(args)
        .env("LC_ALL", "C")
        .stdout(file.try_clone()?)
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()?;
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return Err(io::Error::other(format!("{program} exited with {status}")));
                }
                use std::io::{Read, Seek, SeekFrom};
                let mut file = file;
                file.seek(SeekFrom::Start(0))?;
                let mut text = String::new();
                file.take(1_048_576).read_to_string(&mut text)?;
                return Ok(text);
            }
            Ok(None) if started.elapsed() < timeout => thread::sleep(Duration::from_millis(20)),
            result => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(result.err().unwrap_or_else(|| {
                    io::Error::new(io::ErrorKind::TimedOut, format!("{program} timed out"))
                }));
            }
        }
    }
}

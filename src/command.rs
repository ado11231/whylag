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

pub fn available(program: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::env::var_os("PATH").is_some_and(|path| {
        std::env::split_paths(&path).any(|directory| {
            std::fs::metadata(directory.join(program)).is_ok_and(|metadata| {
                metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
            })
        })
    })
}

pub fn check_tools(ipv6: bool) -> io::Result<()> {
    let required = if cfg!(target_os = "macos") {
        vec![if ipv6 { "ping6" } else { "ping" }, "route", "dscacheutil"]
    } else {
        vec!["ping", "ip", "getent"]
    };
    for program in required {
        if !available(program) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("required command not found: {program}"),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kills_timed_out_commands() {
        let started = Instant::now();
        let error =
            output("sleep", &["5"], Duration::from_millis(60)).expect_err("sleep should time out");
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        assert!(started.elapsed() < Duration::from_secs(2));
    }
    #[test]
    fn reads_output_and_reports_failure() {
        assert_eq!(
            output("printf", &["hello"], Duration::from_secs(1)).expect("printf output"),
            "hello"
        );
        assert!(output("false", &[], Duration::from_secs(1)).is_err());
    }
}

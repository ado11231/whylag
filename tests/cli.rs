use std::process::Command;

#[test]
fn help_and_version_work_without_a_terminal() {
    for option in ["--help", "--version"] {
        let result = Command::new(env!("CARGO_BIN_EXE_whylag"))
            .arg(option)
            .output()
            .expect("run whylag");
        assert!(result.status.success());
        assert!(String::from_utf8_lossy(&result.stdout).contains("whylag"));
    }
}

#[test]
fn invalid_target_has_a_short_error() {
    let result = Command::new(env!("CARGO_BIN_EXE_whylag"))
        .args(["--target", "invalid"])
        .output()
        .expect("run whylag");
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("must be an IPv4 or IPv6 address"));
    assert!(result.stdout.is_empty());
}

#[test]
fn redirected_input_does_not_enter_raw_mode() {
    let result = Command::new(env!("CARGO_BIN_EXE_whylag"))
        .output()
        .expect("run whylag");
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("interactive terminal"));
    assert!(result.stdout.is_empty());
}

#[test]
fn log_preserves_existing_files_and_leaves_missing_replies_blank() {
    use whylag::log::Log;
    let path = std::env::temp_dir().join(format!("whylag-log-test-{}.csv", std::process::id()));
    let mut log = Log::create(&path).expect("new log file");
    log.record("1.1.1.1".parse().expect("target"), None, None, Some(12.5))
        .expect("write sample");
    assert!(Log::create(&path).is_err());
    drop(log);
    let text = std::fs::read_to_string(&path).expect("read log");
    std::fs::remove_file(path).expect("remove test log");
    assert_eq!(text.lines().count(), 2);
    assert!(
        text.lines()
            .nth(1)
            .is_some_and(|line| line.ends_with(",1.1.1.1,,,12.500"))
    );
}

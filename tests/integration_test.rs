use std::process::Command;

fn run_test_echo_through_filter() -> Vec<String> {
    let output = Command::new(env!("CARGO_BIN_EXE_opensmtpd-filter-reject421"))
        .arg(env!("CARGO_BIN_EXE_test_echo"))
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).expect("stdout is not UTF-8");
    stdout.lines().map(str::to_owned).collect()
}

#[test]
fn no_args_exits_with_error() {
    let status = Command::new(env!("CARGO_BIN_EXE_opensmtpd-filter-reject421"))
        .status()
        .expect("failed to execute process");
    assert!(!status.success());
}

#[test]
fn filter_550_becomes_999() {
    let lines = run_test_echo_through_filter();
    assert!(lines.contains(&"filter-result|abc123|tok456|reject|999 Error".to_owned()));
    assert!(!lines.contains(&"filter-result|abc123|tok456|reject|550 some reason".to_owned()));
}

#[test]
fn non_550_reject_passes_through() {
    let lines = run_test_echo_through_filter();
    assert!(lines.contains(&"filter-result|abc123|tok456|reject|421 some reason".to_owned()));
}

#[test]
fn non_filter_result_line_passes_through() {
    let lines = run_test_echo_through_filter();
    assert!(lines.contains(&"some other line".to_owned()));
}

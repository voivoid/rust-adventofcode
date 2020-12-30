fn get_input_path(problem: &str) -> std::path::PathBuf {
    let mut input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/input/");
    input_path.push(problem);

    input_path
}

fn test_problem(problem: &str, input_file: &str, expected_result: &str) {
    let app_path = std::path::PathBuf::from(env!("CARGO_BIN_EXE_rust_aoc"));
    let input_path = get_input_path(input_file);

    let input_file = std::fs::File::open(input_path).expect("failed to open the test input");

    let output = std::process::Command::new(app_path)
        .args(&[problem])
        .stdin(input_file)
        .output()
        .expect("failed to run the test");

    let output_str =
        std::str::from_utf8(&output.stdout[..]).expect("failed to get the test output");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(format!("{}\n", expected_result), output_str);
}

#[test]
fn test_2015_01() {
    test_problem("2015_01_1", "2015/01", "280");
    test_problem("2015_01_2", "2015/01", "1797");
}

#[test]
fn test_2015_02() {
    test_problem("2015_02_1", "2015/02", "1586300");
    test_problem("2015_02_2", "2015/02", "3737498");
}

#[test]
fn test_2015_03() {
    test_problem("2015_03_1", "2015/03", "2592");
    test_problem("2015_03_2", "2015/03", "2360");
}

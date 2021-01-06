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
    assert_eq!(expected_result, output_str.trim());
}

fn test_problems(input_file: &str, expected_result_1: &str, expected_result_2: &str) {
    let problem = input_file.replace("/", "_");
    let problem_a = format!("{}_a", problem);
    let problem_b = format!("{}_b", problem);

    test_problem(problem_a.as_str(), input_file, expected_result_1);
    test_problem(problem_b.as_str(), input_file, expected_result_2);
}

#[test]
fn test_2015_01() {
    test_problems("2015/01", "280", "1797");
}

#[test]
fn test_2015_02() {
    test_problems("2015/02", "1586300", "3737498");
}

#[test]
fn test_2015_03() {
    test_problems("2015/03", "2592", "2360");
}

#[test]
fn test_2015_05() {
    test_problems("2015/05", "255", "55");
}

#[test]
fn test_2015_06() {
    test_problems("2015/06", "569999", "17836115");
}

#[test]
fn test_2015_07() {
    test_problems("2015/07", "46065", "14134");
}

#[test]
fn test_2020_01() {
    test_problems("2020/01", "744475", "70276940");
}

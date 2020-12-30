fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem = args.get(1).expect("Problem is not specified");

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    use rust_aoc::aoc::year_2015;
    let result = match &problem[..] {
        "2015_01_1" => year_2015::problem_01::solve_a(stdin).to_string(),
        "2015_01_2" => year_2015::problem_01::solve_b(stdin).to_string(),
        "2015_02_1" => year_2015::problem_02::solve_a(stdin).to_string(),
        "2015_02_2" => year_2015::problem_02::solve_b(stdin).to_string(),
        "2015_03_1" => year_2015::problem_03::solve_a(stdin).to_string(),
        "2015_03_2" => year_2015::problem_03::solve_b(stdin).to_string(),
        _ => panic!("Unknown problem"),
    };

    println!("{}", result);
}

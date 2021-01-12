fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem = args.get(1).expect("Problem is not specified");

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    use rust_aoc::year_2015;
    use rust_aoc::year_2020;
    let result = match &problem[..] {
        "2015_01_a" => year_2015::problem_01::solve_a(stdin).to_string(),
        "2015_01_b" => year_2015::problem_01::solve_b(stdin).to_string(),
        "2015_02_a" => year_2015::problem_02::solve_a(stdin).to_string(),
        "2015_02_b" => year_2015::problem_02::solve_b(stdin).to_string(),
        "2015_03_a" => year_2015::problem_03::solve_a(stdin).to_string(),
        "2015_03_b" => year_2015::problem_03::solve_b(stdin).to_string(),
        "2015_05_a" => year_2015::problem_05::solve_a(stdin).to_string(),
        "2015_05_b" => year_2015::problem_05::solve_b(stdin).to_string(),
        "2015_06_a" => year_2015::problem_06::solve_a(stdin).to_string(),
        "2015_06_b" => year_2015::problem_06::solve_b(stdin).to_string(),
        "2015_07_a" => year_2015::problem_07::solve_a(stdin).to_string(),
        "2015_07_b" => year_2015::problem_07::solve_b(stdin).to_string(),
        "2015_08_a" => year_2015::problem_08::solve_a(stdin).to_string(),
        "2015_08_b" => year_2015::problem_08::solve_b(stdin).to_string(),
        "2015_09_a" => year_2015::problem_09::solve_a(stdin).to_string(),
        "2015_09_b" => year_2015::problem_09::solve_b(stdin).to_string(),

        "2020_01_a" => year_2020::problem_01::solve_a(stdin).to_string(),
        "2020_01_b" => year_2020::problem_01::solve_b(stdin).to_string(),
        _ => panic!("Unknown problem"),
    };

    println!("{}", result);
}

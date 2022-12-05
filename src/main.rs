mod days;
use clap::Parser;
use days::*;

fn main() {
    let args = Cli::parse();
    match args.generate {
        true => generate_day_template(args.day),
        false => solve(args.day, args.part),
    }
}

fn input_for_day(day: i32) -> String {
    let path =
        "./src/days/day".to_string() + &day.to_string().to_owned() + &"/input.txt".to_owned();
    println!("{path}");
    std::fs::read_to_string(path).unwrap()
}

fn get_int(prompt: &str) -> i32 {
    println!("{prompt}");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn solve(day: i32, part: i32) {
    let input = input_for_day(day);
    let solver: fn(String) -> () = match (day, part) {
        (1, 1) => day1::part1::solve,
        (1, 2) => day1::part2::solve,
        (2, 1) => day2::part1::solve,
        (2, 2) => day2::part1::solve,
        (3, 1) => day3::part2::solve,
        (3, 2) => day3::part2::solve,
        (4, 1) => day4::part2::solve,
        (4, 2) => day4::part2::solve,
        (_, _) => |_| (),
    };
    solver(input);
}

#[derive(Parser)]
struct Cli {
    day: i32,
    part: i32,

    #[arg(short, long)]
    generate: bool,
}

fn generate_day_template(day: i32) {
    let day_dir = "./src/days/day".to_owned() + &day.to_string();
    if !std::path::Path::new(&day_dir).exists() {
        std::fs::create_dir("./src/days/day".to_owned() + &day.to_string()).unwrap();
        let paths = std::fs::read_dir("./template").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let source = path.display().to_string();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let dest = "./src/days/day".to_owned() + &day.to_string() + "/" + &file_name;
            std::fs::copy(source, dest).unwrap();
        }
    }
}

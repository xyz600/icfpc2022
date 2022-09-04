mod common_solver;
mod solver1;
mod solver2;

use clap::{App, Arg};
use common::problem::Image;

use solver2::solve;

fn main() {
    let app = App::new("xyzsolver").version("1").author("xyz600").about("problem solver for icfpc 2022").arg(
        Arg::with_name("problem-id")
            .help("input problem id")
            .short('i')
            .long("problem-id")
            .required(true)
            .takes_value(true),
    );

    let matches = app.get_matches();

    let problem_id = matches.value_of("problem-id").unwrap().parse::<usize>().unwrap();
    let input_filepath = format!("dataset/{}.png", problem_id);
    let image = Image::new(input_filepath.as_str());

    let final_state = solve(problem_id, &image);
    final_state.print_output();
}

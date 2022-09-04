mod common_solver;
mod mincost_matching;
mod solver1;
mod solver2;
mod solver3;

use clap::{App, Arg};
use common::problem::Image;

fn main() {
    let app = App::new("xyzsolver")
        .version("1")
        .author("xyz600")
        .about("problem solver for icfpc 2022")
        .arg(Arg::with_name("problem-id").help("input problem id").short('i').long("problem-id").required(true).takes_value(true))
        .arg(
            Arg::with_name("solver-type")
                .help("select solver type to use. set 1 | 2 | 3 (solver 3 is only available when use-twin-image is on.)")
                .short('s')
                .long("solver-type")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("use-twin-image")
                .help("use twin image for problem 26..35")
                .short('t')
                .long("use-twin-image")
                .action(clap::ArgAction::SetTrue),
        );

    let matches = app.get_matches();

    let problem_id = matches.value_of("problem-id").unwrap().parse::<usize>().unwrap();
    let input_filepath = format!("dataset/{}.png", problem_id);
    let image = Image::new(input_filepath.as_str());

    let use_twin_image = matches.get_flag("use-twin-image");

    let solver_type = matches.value_of("solver-type").unwrap();

    let final_state = if use_twin_image {
        solver3::solve(problem_id, &image)
    } else {
        if solver_type == "1" {
            solver1::solve(problem_id, &image)
        } else if solver_type == "2" {
            solver2::solve(problem_id, &image)
        } else {
            panic!("unknown solver");
        }
    };
    final_state.save_image(&format!("solution/img/{problem_id}.png"));
    final_state.print_output();
}

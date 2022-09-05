mod common_solver;
mod mincost_matching;
mod solver1;
mod solver2;
mod solver3;
mod solver4;
mod solver5;
mod solver6;
mod solver7;

use std::path::Path;

use clap::{App, Arg};
use common::problem::{evaluate, Image, StateWithScore};

fn main() {
    let app = App::new("xyzsolver")
        .version("1")
        .author("xyz600")
        .about("problem solver for icfpc 2022")
        .arg(Arg::with_name("problem-id").help("input problem id").short('i').long("problem-id").required(true).takes_value(true))
        .arg(
            Arg::with_name("solver-type")
                .help("select solver type to use. set 1 | 2 | 3 | 4 | 5 | 6 | 7 (solver 3 | 4 | 5 is only available when use-twin-image is on.)")
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
        if solver_type == "3" {
            solver3::solve(problem_id, &image)
        } else if solver_type == "4" {
            solver4::solve(problem_id, &image)
        } else if solver_type == "5" {
            solver5::solve(problem_id, &image)
        } else {
            panic!("unknown solver");
        }
    } else {
        if solver_type == "1" {
            solver1::solve(problem_id, &image)
        } else if solver_type == "2" {
            solver2::solve(problem_id, &image)
        } else if solver_type == "6" {
            solver6::solve(problem_id, &image)
        } else if solver_type == "7" {
            assert_eq!(problem_id, 1);
            solver7::solve(problem_id, &image)
        } else {
            panic!("unknown solver");
        }
    };
    final_state.save_image(&format!("solution/img/{problem_id}.png"));
    final_state.print_output(Path::new(&format!("solution/{problem_id}.txt",)));

    let score = evaluate(&image, &final_state);
    StateWithScore { score, state: final_state }.save_if_global_best(problem_id);
}

use crate::common_solver::{calculate_divisor_list, detect_edge, solve_by_divisor};
use common::problem::*;

pub fn solve(problem_id: usize, image: &Image) -> State {
    const POS_THREASHOLD: usize = 50;

    let mut best_state = State::new(image.height, image.width);
    let mut best_score = evaluate(image, &best_state);

    {
        // edge 検出して、パターン数が少なければやってみる
        let (row_list, column_list) = detect_edge(image, 20.0);
        if row_list.len() <= POS_THREASHOLD && column_list.len() <= POS_THREASHOLD {
            eprintln!("trying edge based division");
            eprintln!("row {:?}", row_list);
            eprintln!("col {:?}", column_list);
            let state = solve_by_divisor(image, &row_list, &column_list);
            let exact_score = evaluate(image, &state);
            eprintln!("update: {} -> {}", best_score, exact_score);

            if best_score > exact_score {
                best_score = exact_score;
                best_state = state;

                StateWithScore {
                    score: best_score,
                    state: best_state.clone(),
                }
                .save_if_global_best(problem_id);
            }
        } else {
            eprintln!("cannot solve with edge: row = {}, column = {}", row_list.len(), column_list.len());
        }
    }

    {
        let image_size = 400;
        let step_list = calculate_divisor_list(image_size);
        eprintln!("step_list: {:?}", step_list);

        for step in step_list.into_iter() {
            if image_size > POS_THREASHOLD * step {
                eprintln!("skip because step is too small ... {}", step);
                continue;
            }
            eprintln!("trying {}", step);

            let mut column_list = vec![];
            let mut row_list = vec![];

            for i in (0..=image_size).step_by(step) {
                column_list.push(i);
                row_list.push(i);
            }

            let state = solve_by_divisor(image, &row_list, &column_list);
            let exact_score = evaluate(image, &state);
            eprintln!("update: {} -> {}", best_score, exact_score);
            if best_score > exact_score {
                best_score = exact_score;
                best_state = state;
                StateWithScore {
                    score: best_score,
                    state: best_state.clone(),
                }
                .save_if_global_best(problem_id);
            }
        }
    }
    best_state
}

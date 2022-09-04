use crate::common_solver::{calculate_divisor_list, detect_edge, solve_by_divisor};
use common::problem::*;

pub fn solve(problem_id: usize, image: &Image) -> State {
    const POS_THREASHOLD: usize = 25;

    let mut best_state = State::new(image.height, image.width);
    let mut best_score = evaluate(image, &best_state);

    // 最大長方形を見つける
    // 塗りつぶす
    // その辺から row_list, column_list を構築する

    best_state
}

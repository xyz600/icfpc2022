use std::path::Path;

use common::{cumulative_sum::RangeColorMedianCalculator, problem::*};

pub fn solve(problem_id: usize, image: &Image) -> State {
    let str_path = format!("dataset/{}.initial.json", problem_id);
    let config_path = Path::new(&str_path);
    let config = common::config_loader::TwinImageConfig::load(config_path);

    let mut state = State::create_with_config(&config);

    let median_calculator = RangeColorMedianCalculator::new(image);

    for block_index in 0..state.block_list.len() {
        if state.block_list[block_index].is_child {
            // median の色を塗る
            let rect = state.block_list[block_index].rect;
            let color = median_calculator.median(rect.bottom(), rect.left(), rect.top(), rect.right());
            state.apply(Command::Color(block_index, color));
        }
    }

    state
}

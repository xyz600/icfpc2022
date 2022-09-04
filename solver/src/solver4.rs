use common::{cumulative_sum::RangeColorMedianCalculator, problem::*};
use std::path::Path;

pub fn solve(problem_id: usize, image: &Image) -> State {
    let str_path = format!("dataset/{}.initial.json", problem_id);
    let config_path = Path::new(&str_path);
    let config = common::config_loader::TwinImageConfig::load(config_path);

    let state = State::create_with_config(&config);

    let median_calculator = RangeColorMedianCalculator::new(image);

    let unit = state.block_list.last().unwrap().rect.height;
    let dim = image.height / unit;

    let mut color_buffer = vec![vec![Color8::new(0, 0, 0, 0); dim]; dim];
    for y in 0..dim {
        for x in 0..dim {
            color_buffer[y][x] = median_calculator.median(y * unit, x * unit, (y + 1) * unit, (x + 1) * unit);
        }
    }
    let mut block_index_table = vec![vec![0; dim]; dim];
    for block in state.block_list.iter() {
        let iy = block.rect.bottom() / unit;
        let ix = block.rect.left() / unit;
        block_index_table[iy][ix] = block.index_of;
    }

    // 横へ見て行って、同じ色なら merge
    let mut state_x = state.clone();
    for yi in 0..dim {
        let mut xi = 0;
        while xi < dim {
            let start = xi;
            let mut merge_block_from = block_index_table[yi][xi];
            let mut index = start;
            while index < dim - 1 && color_buffer[yi][index] == color_buffer[yi][index + 1] {
                let merge_block_to = block_index_table[yi][index + 1];
                state_x.apply(Command::Merge(merge_block_from, merge_block_to));
                // merge すると最後にブロックが追加されるので、それと次に merge する
                merge_block_from = state_x.block_list.len() - 1;
                index += 1;
            }
            state_x.apply(Command::Color(merge_block_from, color_buffer[yi][xi]));
            xi = index + 1;
        }
    }

    // 縦へ見て行って、同じ色なら merge
    let mut state_y = state.clone();
    for xi in 0..dim {
        let mut yi = 0;
        while yi < dim {
            let start = yi;
            let mut merge_block_from = block_index_table[yi][xi];
            let mut index = start;
            while index < dim - 1 && color_buffer[index][xi] == color_buffer[index + 1][xi] {
                let merge_block_to = block_index_table[index + 1][xi];
                state_y.apply(Command::Merge(merge_block_from, merge_block_to));
                // merge すると最後にブロックが追加されるので、それと次に merge する
                merge_block_from = state_y.block_list.len() - 1;
                index += 1;
            }
            state_y.apply(Command::Color(merge_block_from, color_buffer[yi][xi]));
            yi = index + 1;
        }
    }

    let cost_x = evaluate(image, &state_x);
    let cost_y = evaluate(image, &state_y);
    if cost_x < cost_y {
        state_x
    } else {
        state_y
    }
}

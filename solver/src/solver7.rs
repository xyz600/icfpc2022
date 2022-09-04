use common::{cumulative_sum::RangeColorMedianCalculator, problem::*};

use crate::common_solver::{calculate_divisor_list, detect_edge};

pub fn solve(problem_id: usize, image: &Image) -> State {
    let median_calculator = RangeColorMedianCalculator::new(image);

    let (row_list, column_list) = detect_edge(image, 30.0);

    let mut state = State::new(image.height, image.width);

    let blue_color = median_calculator.median(row_list[0], column_list[0], row_list[1], column_list[10]);
    let white_color = median_calculator.median(row_list[9], column_list[0], row_list[10], column_list[1]);
    let black_color = median_calculator.median(row_list[9], column_list[1], row_list[10], column_list[2]);

    state.apply(Command::HorizontalSplit(0, row_list[1]));
    state.apply(Command::Color(1, blue_color));

    state.apply(Command::VerticalSplit(2, column_list[9]));
    state.apply(Command::Color(4, blue_color));

    // 横1列の block の index
    let mut single_block_index_table = vec![0; 10];
    {
        let mut split_target_block_id = 3;
        for yi in 2..10 {
            state.apply(Command::HorizontalSplit(split_target_block_id, row_list[yi]));
            single_block_index_table[yi - 1] = state.block_list.len() - 2;
            split_target_block_id = state.block_list.len() - 1;
        }
        single_block_index_table[9] = state.block_list.len() - 1;
    }

    // 横2列の block の index
    let mut dual_block_index_table = vec![vec![0, 0]; 10];
    {
        for yi in 1..10 {
            state.apply(Command::VerticalSplit(single_block_index_table[yi], column_list[4]));

            let block_index = if yi % 2 == 1 { state.block_list.len() - 2 } else { state.block_list.len() - 1 };
            state.apply(Command::Color(block_index, black_color));
            dual_block_index_table[yi][0] = state.block_list.len() - 2;
            dual_block_index_table[yi][1] = state.block_list.len() - 1;
        }
    }
    eprintln!("{:?}", dual_block_index_table);

    return state;

    let mut single_column_block_index_table = vec![0; 10];
    // 左列 merge
    {
        let mut merge_target_block_id = dual_block_index_table[1][0];
        for yi in 2..10 {
            state.apply(Command::Merge(merge_target_block_id, dual_block_index_table[yi - 1][0]));
            merge_target_block_id = state.block_list.len() - 1;
        }

        let mut target_block_id = state.block_list.len() - 1;
        for xi in 1..4 {
            state.apply(Command::VerticalSplit(target_block_id, column_list[xi]));
            target_block_id = state.block_list.len() - 1;
            single_column_block_index_table[xi - 1] = state.block_list.len() - 2;
        }
        single_column_block_index_table[4] = state.block_list.len() - 1;
    }

    return state;

    // 右列 merge
    {
        let mut merge_target_block_id = dual_block_index_table[9][1];
        for yi in (2..9).rev() {
            state.apply(Command::Merge(merge_target_block_id, dual_block_index_table[yi][1]));
            merge_target_block_id = state.block_list.len() - 1;
        }
        let right_all_block_id = merge_target_block_id;

        // 1マスだけ青で塗る
        state.apply(Command::VerticalSplit(dual_block_index_table[1][1], column_list[8]));
        let small_block_index = state.block_list.len() - 1;
        let lower_left_block_index = state.block_list.len() - 2;
        state.apply(Command::Color(small_block_index, blue_color));

        // 一番右を切って、下を merge
        state.apply(Command::VerticalSplit(right_all_block_id, column_list[8]));

        let rest_block_index = state.block_list.len() - 1;
        state.apply(Command::Merge(rest_block_index, lower_left_block_index));

        // 縦切断
        let mut target_block_id = state.block_list.len() - 1;
        for xi in 5..8 {
            state.apply(Command::VerticalSplit(target_block_id, column_list[xi]));
            target_block_id = state.block_list.len() - 1;
            // 縦列の位置をメモ
            single_column_block_index_table[xi - 1] = state.block_list.len() - 2;
        }
        single_column_block_index_table[8] = state.block_list.len() - 1;
    }

    state.apply(Command::Swap(single_column_block_index_table[0], single_column_block_index_table[5]));
    state.apply(Command::Swap(single_column_block_index_table[2], single_column_block_index_table[7]));

    state
}

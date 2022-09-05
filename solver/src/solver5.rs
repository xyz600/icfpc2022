use common::problem::*;
use std::path::Path;

use crate::{
    common_solver::{calculate_divisor_list, detect_edge, solve_by_divisor},
    solver2,
};

// 全て merge して、 solver2::solve と同じことをします
pub fn solve(problem_id: usize, image: &Image) -> State {
    let str_path = format!("dataset/{}.initial.json", problem_id);
    let config_path = Path::new(&str_path);
    let config = common::config_loader::TwinImageConfig::load(config_path);

    let mut state = State::create_with_config(&config);

    let unit = state.block_list.last().unwrap().rect.height;
    let dim = image.height / unit;

    let mut block_index_table = vec![vec![0; dim]; dim];
    for block in state.block_list.iter() {
        let iy = block.rect.bottom() / unit;
        let ix = block.rect.left() / unit;
        block_index_table[iy][ix] = block.index_of;
    }

    let mut row_block_index = vec![0; dim];
    for yi in 0..dim {
        let mut block_index_from = block_index_table[yi][0];
        for xi in 1..dim {
            state.apply(Command::Merge(block_index_from, block_index_table[yi][xi]));
            block_index_from = state.block_list.len() - 1;
        }
        row_block_index[yi] = state.block_list.len() - 1;
    }
    {
        let mut block_index_from = row_block_index[0];
        for yi in 1..dim {
            state.apply(Command::Merge(block_index_from, row_block_index[yi]));
            block_index_from = state.block_list.len() - 1;
        }
    }
    // 初期状態に戻す
    state.apply(Command::Color(state.block_list.len() - 1, Color8::new(255, 255, 255, 255)));

    // almost all solver2
    // ただし、solve させた後コマンドの移し替えをする

    let apply_prestate = |s: State| -> State {
        let mut clone = state.clone();
        let offset = clone.block_list.len() - 1;
        for cmd in s.get_command_list().into_iter() {
            match cmd {
                Command::HorizontalSplit(block_index, y) => clone.apply(Command::HorizontalSplit(block_index + offset, y)),
                Command::VerticalSplit(block_index, x) => clone.apply(Command::VerticalSplit(block_index + offset, x)),
                Command::PointSplit(block_index, pos) => clone.apply(Command::PointSplit(block_index + offset, pos)),
                Command::Color(block_index, color) => clone.apply(Command::Color(block_index + offset, color)),
                Command::Swap(block_index1, block_index2) => clone.apply(Command::Swap(block_index1 + offset, block_index2 + offset)),
                Command::Merge(block_index1, block_index2) => clone.apply(Command::Merge(block_index1 + offset, block_index2 + offset)),
            }
        }
        clone
    };

    // コピペしないと、途中の解が invalid になるのでダメでした
    const POS_THREASHOLD: usize = 50;

    let mut best_state = state.clone();
    let mut best_score = evaluate(image, &best_state);

    {
        // edge 検出して、パターン数が少なければやってみる
        let (row_list, column_list) = detect_edge(image, 20.0);
        if row_list.len() <= POS_THREASHOLD && column_list.len() <= POS_THREASHOLD {
            eprintln!("trying edge based division");
            eprintln!("row {:?}", row_list);
            eprintln!("col {:?}", column_list);
            let pre_state = solve_by_divisor(image, &row_list, &column_list);
            let state = apply_prestate(pre_state);

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

            let pre_state = solve_by_divisor(image, &row_list, &column_list);
            let state = apply_prestate(pre_state);

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

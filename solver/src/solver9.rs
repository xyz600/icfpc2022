use common::{intset::IntSet, problem::*, random::CachedRandom};
use std::{path::Path, time::Instant};

use crate::{
    common_solver::{self, calculate_divisor_list, detect_edge, solve_by_divisor},
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

    // solver 6 のコピペ
    // コピペしないと、途中の解が invalid になるのでダメでした

    let init_state = if let Some(v) = StateWithScore::load(problem_id) {
        v.state
    } else {
        solver2::solve(problem_id, image)
    };

    let mut row_list = vec![];
    let mut column_list = vec![];

    for block in init_state.block_list.iter() {
        if !block.is_child {
            continue;
        }
        let rect = block.rect;
        if !row_list.contains(&rect.bottom()) {
            row_list.push(rect.bottom());
        }
        if !row_list.contains(&(rect.top() + 1)) {
            row_list.push(rect.top() + 1);
        }
        if !column_list.contains(&rect.left()) {
            column_list.push(rect.left());
        }
        if !column_list.contains(&(rect.right() + 1)) {
            column_list.push(rect.right() + 1);
        }
    }
    row_list.sort();
    column_list.sort();

    let mut best_eval = evaluate(image, &init_state);
    let mut best_state = init_state;

    let mut rand = CachedRandom::new(65535, 0);

    loop {
        let candidate_size = row_list.len() + column_list.len() - 4;
        let mut dlb = IntSet::new(candidate_size);
        for i in 0..candidate_size {
            dlb.add(i);
        }

        let mut finish = false;

        let mut turn = 0;
        while !dlb.is_empty() {
            eprintln!("start turn {}, dlb size: {}", turn, dlb.size());
            turn += 1;

            let root_index = dlb.choose(&mut rand);

            let select_column = root_index < column_list.len() - 2;
            let index = if select_column { root_index + 1 } else { root_index - (column_list.len() - 2) + 1 } as usize;

            let mut success = false;
            for offset in [-1, 1].into_iter() {
                if select_column {
                    if offset > 0 && column_list[index + 1] - column_list[index] <= 2 {
                        continue;
                    }
                    if offset < 0 && column_list[index] - column_list[index - 1] <= 2 {
                        continue;
                    }
                    column_list[index] = (column_list[index] as i64 + offset) as usize;
                } else {
                    if offset > 0 && row_list[index + 1] - row_list[index] <= 2 {
                        continue;
                    }
                    if offset < 0 && row_list[index] - row_list[index - 1] <= 2 {
                        continue;
                    }
                    row_list[index] = (row_list[index] as i64 + offset) as usize;
                }

                let start = Instant::now();
                let pre_state = common_solver::solve_by_divisor(image, &row_list, &column_list);
                let state = apply_prestate(pre_state);

                let elapsed = (Instant::now() - start).as_secs();
                if elapsed > 20 {
                    finish = true;
                }

                let eval = evaluate(image, &state);

                if best_eval > eval {
                    success = true;
                    eprintln!("update! {} -> {}", best_eval, eval);
                    eprintln!("    column: {:?}", column_list);
                    eprintln!("    row: {:?}", row_list);
                    best_eval = eval;
                    best_state = state;

                    StateWithScore {
                        score: best_eval,
                        state: best_state.clone(),
                    }
                    .save_if_global_best(problem_id);

                    break;
                } else {
                    if select_column {
                        column_list[index] = (column_list[index] as i64 - offset) as usize;
                    } else {
                        row_list[index] = (row_list[index] as i64 - offset) as usize;
                    }
                }
            }

            if !success {
                dlb.remove(root_index);
            } else {
                if 0 < root_index {
                    dlb.add(root_index - 1);
                }
                if root_index < candidate_size - 1 {
                    dlb.add(root_index + 1);
                }
            }
        }

        if finish {
            break;
        }

        eprintln!("add random edge...");
        // ランダムに1 本辺を追加する
        let select_column = rand.next_float() < 0.5;
        if select_column {
            let mut column = rand.next_int_range(1, 400 - 1) as usize;
            loop {
                let mut success = true;
                for v in column_list.iter() {
                    if column.abs_diff(*v) < 3 {
                        success = false;
                    }
                }
                if success {
                    break;
                }
                column = rand.next_int_range(1, 400 - 1) as usize;
            }
            column_list.push(column);
            column_list.sort();
        } else {
            let mut row = rand.next_int_range(1, 400 - 1) as usize;
            loop {
                let mut success = true;
                for v in row_list.iter() {
                    if row.abs_diff(*v) < 3 {
                        success = false;
                    }
                }
                if success {
                    break;
                }
                row = rand.next_int_range(1, 400 - 1) as usize;
            }
            row_list.push(row);
            row_list.sort();
        }
    }

    best_state
}

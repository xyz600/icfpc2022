use crate::{common_solver, solver2};
use common::{intset::IntSet, problem::*, random::CachedRandom};

/// row や column を ±1 して調整する山登り
/// 本当は敷居を増やすのも効果はかなりある（このルールだと損しないので）けど、
/// 時間を見てかなー
pub fn solve(problem_id: usize, image: &Image) -> State {
    let init_state = solver2::solve(problem_id, image);

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
    let candidate_size = row_list.len() + column_list.len() - 4;
    let mut dlb = IntSet::new(candidate_size);
    for i in 0..candidate_size {
        dlb.add(i);
    }

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

            let state = common_solver::solve_by_divisor(image, &row_list, &column_list);
            let eval = evaluate(image, &state);

            if best_eval > eval {
                success = true;
                eprintln!("update! {} -> {}", best_eval, eval);
                eprintln!("    column: {:?}", column_list);
                eprintln!("    row: {:?}", row_list);
                best_eval = eval;
                best_state = state;
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
        }
    }

    best_state
}

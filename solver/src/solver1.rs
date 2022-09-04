use common::cumulative_sum::*;
use common::problem::*;

fn detect_edge(image: &Image, threashold: f64) -> (Vec<usize>, Vec<usize>) {
    let mut row_edge_list = vec![];

    for y in 0..image.height - 1 {
        for x in 0..image.width {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y + 1, x).to64();
            if (c1 - c2).abs().horizontal_add() > threashold {
                row_edge_list.push(y);
                break;
            }
        }
    }

    let mut col_edge_list = vec![];
    for x in 0..image.width - 1 {
        for y in 0..image.height {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y, x + 1).to64();
            if (c1 - c2).abs().horizontal_add() > threashold {
                col_edge_list.push(x);
                break;
            }
        }
    }

    (row_edge_list, col_edge_list)
}

fn detect_corner(image: &Image, threashold: f64) -> Vec<Pos> {
    let mut ret = vec![];
    for y in 1..image.height - 1 {
        for x in 1..image.width - 1 {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y, x + 1).to64();
            let c3 = image.color_of(y + 1, x).to64();
            let c4 = image.color_of(y + 1, x + 1).to64();

            let diff_hor = (c1 - c2).abs().horizontal_add() + (c3 - c4).abs().horizontal_add();
            let diff_vert = (c1 - c3).abs().horizontal_add() + (c2 - c4).abs().horizontal_add();
            if diff_hor.min(diff_vert) > threashold {
                ret.push(Pos::new(y, x));
            }
        }
    }
    ret
}

pub fn solve(image: &Image) -> State {
    let cum = CumulativeRMSESum::new(image);

    let init_state = State::new(image.height, image.width);

    let (row_edge_list, coloumn_edge_list) = detect_edge(image, 30.0);
    let corner_list = detect_corner(image, 30.0);

    let mut buffer_list = [vec![init_state.clone()], vec![]];
    const BEAM_WIDTH: usize = 102;
    assert!(BEAM_WIDTH % 3 == 0);

    let mut best_state = init_state;
    let mut best_eval = evaluate(image, &best_state);
    let mut last_updated_turn = 0;

    for turn in 0.. {
        eprintln!("start turn {}", turn);

        let mut diff_list = vec![];

        for state_index in 0..buffer_list[turn % 2].len() {
            let state = &buffer_list[turn % 2][state_index];

            for block_index in 0..state.block_list.len() {
                if !state.block_list[block_index].is_child {
                    continue;
                }
                let rect = state.block_list[block_index].rect;
                let before_rmse = cum.range_rmse(rect.bottom(), rect.left(), rect.top() + 1, rect.right() + 1).horizontal_add();
                // 貪欲法
                // 以下の選択肢で一番良さそうなものを選択
                // 1. 横線1本 + 2色
                for y in row_edge_list.iter() {
                    let y = *y;
                    if rect.bottom() < y && y < rect.top() {
                        let after_rmse =
                            cum.range_rmse(rect.bottom(), rect.left(), y, rect.right() + 1) + cum.range_rmse(y, rect.left(), rect.top() + 1, rect.right() + 1);
                        let gain = before_rmse - after_rmse.horizontal_add();
                        if 0.0 < gain {
                            diff_list.push((gain, state_index, Command::HorizontalSplit(block_index, y)));
                        }
                    }
                }

                // 2. 縦線1本 + 2色
                for x in coloumn_edge_list.iter() {
                    let x = *x;
                    if rect.left() < x && x < rect.right() {
                        let after_rmse =
                            cum.range_rmse(rect.bottom(), rect.left(), rect.top() + 1, x) + cum.range_rmse(rect.bottom(), x, rect.top() + 1, rect.right() + 1);
                        let gain = before_rmse - after_rmse.horizontal_add();
                        if 0.0 < gain {
                            diff_list.push((gain, state_index, Command::VerticalSplit(block_index, x)));
                        }
                    }
                }

                // 3. 点1つ + 4色
                // FIXME: 候補絞る
                for p in corner_list.iter() {
                    if !rect.is_internal(p) {
                        continue;
                    }
                    let after_rmse = cum.range_rmse(rect.bottom(), rect.left(), p.y, p.x)
                        + cum.range_rmse(rect.bottom(), p.x, p.y, rect.right() + 1)
                        + cum.range_rmse(p.y, p.x, rect.top() + 1, rect.right() + 1)
                        + cum.range_rmse(p.y, rect.left(), rect.top(), p.x);
                    let gain = before_rmse - after_rmse.horizontal_add();
                    if 0.0 < gain {
                        diff_list.push((gain, state_index, Command::PointSplit(block_index, Pos::new(p.y, p.x))));
                    }
                }
            }
        }
        if diff_list.is_empty() {
            break;
        }

        diff_list.sort_by_key(|(v, _, _)| (-*v as i64));

        // FIXME: 高速化
        buffer_list[(turn + 1) % 2].clear();

        let mut command_counter = [0, 0, 0];

        for (_, state_index, command) in diff_list.into_iter() {
            let counter_index = match command {
                Command::HorizontalSplit(_, _) => 0,
                Command::VerticalSplit(_, _) => 1,
                Command::PointSplit(_, _) => 2,
                _ => {
                    panic!("bug")
                }
            };
            if BEAM_WIDTH / 3 <= command_counter[counter_index] {
                continue;
            }
            command_counter[counter_index] += 1;

            let mut state = buffer_list[turn % 2][state_index].clone();
            state.apply(command);

            let rect = state.block_list[command.block_index()].rect;
            match command {
                Command::HorizontalSplit(_, y) => {
                    let new_block_index = state.block_list.len() - 2;
                    let bottom_color = cum.mean_color(rect.bottom(), rect.left(), y, rect.right() + 1);
                    let top_color = cum.mean_color(y, rect.left(), rect.top() + 1, rect.right() + 1);
                    state.apply(Command::Color(new_block_index, bottom_color));
                    state.apply(Command::Color(new_block_index + 1, top_color));
                }
                Command::VerticalSplit(_, x) => {
                    let new_block_index = state.block_list.len() - 2;
                    let left_color = cum.mean_color(rect.bottom(), rect.left(), rect.top() + 1, x);
                    let right_color = cum.mean_color(rect.bottom(), x, rect.top() + 1, rect.right() + 1);
                    state.apply(Command::Color(new_block_index, left_color));
                    state.apply(Command::Color(new_block_index + 1, right_color));
                }
                Command::PointSplit(_, pos) => {
                    let new_block_index = state.block_list.len() - 4;
                    let bl_color = cum.mean_color(rect.bottom(), rect.left(), pos.y, pos.x);
                    let br_color = cum.mean_color(rect.bottom(), pos.x, pos.y, rect.right() + 1);
                    let tr_color = cum.mean_color(pos.y, pos.x, rect.top() + 1, rect.right() + 1);
                    let tl_color = cum.mean_color(pos.y, rect.left(), rect.top() + 1, pos.x);
                    for (index, color) in [bl_color, br_color, tr_color, tl_color].iter().enumerate() {
                        state.apply(Command::Color(new_block_index + index, *color));
                    }
                }
                _ => {
                    panic!("maybe bugs");
                }
            };

            let exact_eval = evaluate(image, &state);
            if best_eval > exact_eval {
                best_eval = exact_eval;
                best_state = state.clone();
                last_updated_turn = 0;
            }
            buffer_list[(turn + 1) % 2].push(state);

            if buffer_list[(turn + 1) % 2].len() == BEAM_WIDTH {
                break;
            }
        }
        last_updated_turn += 1;
        if last_updated_turn == 20 {
            break;
        }
    }
    best_state
}

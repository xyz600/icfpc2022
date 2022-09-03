use clap::{App, Arg};
use common::cumulative_sum::*;
use common::problem::*;

fn detect_edge(image: &Image, threashold: f64) -> (Vec<usize>, Vec<usize>) {
    let mut row_edge_list = vec![];

    for row in 0..image.height - 1 {
        for col in 0..image.width {
            let c1 = image.color_of(row, col).to64();
            let c2 = image.color_of(row + 1, col).to64();
            if (c1 - c2).horizontal_add() > threashold {
                row_edge_list.push(row);
                break;
            }
        }
    }

    let mut col_edge_list = vec![];
    for col in 0..image.width - 1 {
        for row in 0..image.height {
            let c1 = image.color_of(row, col).to64();
            let c2 = image.color_of(row, col + 1).to64();
            if (c1 - c2).horizontal_add() > threashold {
                col_edge_list.push(col);
                break;
            }
        }
    }

    (row_edge_list, col_edge_list)
}

fn solve(image: &Image) -> State {
    let mut state = State::new(image.height, image.width);

    let cum = CumulativeRMSESum::new(image);

    let mut exact_eval = evaluate(image, &state);

    for turn in 0.. {
        eprintln!("---");
        let mut best_command = None;
        let mut best_gain = 0f64;

        for block_index in 0..state.block_list.len() {
            if !state.block_list[block_index].is_child {
                continue;
            }
            let rect = state.block_list[block_index].rect;
            let before_rmse = cum
                .range_rmse(rect.bottom(), rect.left(), rect.top() + 1, rect.right() + 1)
                .horizontal_add();
            // 貪欲法
            // 以下の選択肢で一番良さそうなものを選択
            // 1. 横線1本 + 2色
            for y in rect.bottom() + 1..rect.top() {
                let after_rmse = cum.range_rmse(rect.bottom(), rect.left(), y, rect.right() + 1)
                    + cum.range_rmse(y, rect.left(), rect.top() + 1, rect.right() + 1);
                let gain = before_rmse - after_rmse.horizontal_add();
                if best_gain < gain {
                    best_gain = gain;
                    best_command = Some(Command::HorizontalSplit(block_index, y));
                }
            }

            // 2. 縦線1本 + 2色
            for x in rect.left() + 1..rect.right() {
                let after_rmse = cum.range_rmse(rect.bottom(), rect.left(), rect.top() + 1, x)
                    + cum.range_rmse(rect.bottom(), x, rect.top() + 1, rect.right() + 1);
                let gain = before_rmse - after_rmse.horizontal_add();
                if best_gain < gain {
                    best_gain = gain;
                    best_command = Some(Command::VerticalSplit(block_index, x));
                }
            }

            // 3. 点1つ + 4色
            for y in rect.bottom() + 1..rect.top() {
                for x in rect.left() + 1..rect.right() {
                    let after_rmse = cum.range_rmse(rect.bottom(), rect.left(), y, x)
                        + cum.range_rmse(rect.bottom(), x, y, rect.right() + 1)
                        + cum.range_rmse(y, x, rect.top() + 1, rect.right() + 1)
                        + cum.range_rmse(y, rect.left(), rect.top(), x);
                    let gain = before_rmse - after_rmse.horizontal_add();
                    if best_gain < gain {
                        best_gain = gain;
                        best_command = Some(Command::PointSplit(block_index, Pos::new(y, x)));
                    }
                }
            }
            // FIXME:
            // 4. 横線2本 + 3色
            // 5. 縦線2本 + 3色
        }

        eprintln!("    best_gain = {}", best_gain);
        eprintln!("    command: {:?}", best_command);

        if let Some(command) = best_command {
            state.apply(command);
            let rect = state.block_list[command.block_index()].rect;
            let undo_count = match command {
                Command::HorizontalSplit(_, y) => {
                    let new_block_index = state.block_list.len() - 2;
                    let bottom_color =
                        cum.mean_color(rect.bottom(), rect.left(), y, rect.right() + 1);
                    let top_color =
                        cum.mean_color(y, rect.left(), rect.top() + 1, rect.right() + 1);
                    state.apply(Command::Color(new_block_index, bottom_color));
                    state.apply(Command::Color(new_block_index + 1, top_color));
                    2
                }
                Command::VerticalSplit(_, x) => {
                    let new_block_index = state.block_list.len() - 2;
                    let left_color = cum.mean_color(rect.bottom(), rect.left(), rect.top() + 1, x);
                    let right_color =
                        cum.mean_color(rect.bottom(), x, rect.top() + 1, rect.right() + 1);
                    state.apply(Command::Color(new_block_index, left_color));
                    state.apply(Command::Color(new_block_index + 1, right_color));
                    2
                }
                Command::PointSplit(_, pos) => {
                    let new_block_index = state.block_list.len() - 4;
                    let bl_color = cum.mean_color(rect.bottom(), rect.left(), pos.y, pos.x);
                    let br_color = cum.mean_color(rect.bottom(), pos.x, pos.y, rect.right() + 1);
                    let tr_color = cum.mean_color(pos.y, pos.x, rect.top() + 1, rect.right() + 1);
                    let tl_color = cum.mean_color(pos.y, rect.left(), rect.top() + 1, pos.x);
                    for (index, color) in
                        [bl_color, br_color, tr_color, tl_color].iter().enumerate()
                    {
                        state.apply(Command::Color(new_block_index + index, *color));
                    }
                    4
                }
                _ => {
                    panic!("maybe bugs");
                }
            };
            let next_exact_eval = evaluate(image, &state);
            eprintln!("update {} -> {}", exact_eval, next_exact_eval);
            if next_exact_eval > exact_eval {
                // 厳密コスト計算だけして、色塗りは一番最後に全部やる
                for _iter in 0..undo_count + 1 {
                    state.undo();
                }
                break;
            } else {
                exact_eval = next_exact_eval;
            }
        } else {
            // そもそも色の誤差が減らないなら継続の意味がない
            break;
        }
    }
    state
}

fn main() {
    let app = App::new("xyzsolver")
        .version("1")
        .author("xyz600")
        .about("problem solver for icfpc 2022")
        .arg(
            Arg::with_name("problem-id")
                .help("input problem id")
                .short('i')
                .long("problem-id")
                .required(true)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let problem_id = matches.value_of("problem-id").unwrap();
    let input_filepath = format!("dataset/{}.png", problem_id);
    let image = Image::new(input_filepath.as_str());

    let final_state = solve(&image);
    final_state.save_image(&format!("solution/{problem_id}.png"));
    final_state.print_output();
}

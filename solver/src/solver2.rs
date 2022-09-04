use crate::common_solver::solve_by_divisor;
use common::problem::*;

fn detect_edge(image: &Image, threashold: f64) -> (Vec<usize>, Vec<usize>) {
    let select_value = |score_table: &Vec<f64>| -> Vec<usize> {
        const RANGE_THREASHOLD: usize = 5;

        let mut vs = vec![];
        for i in 0..score_table.len() {
            vs.push((score_table[i], i));
        }
        vs.sort_by_key(|v| (-v.0 * 1000.0) as i64);

        let mut selected = vec![false; score_table.len()];

        let mut ret = vec![];
        for (_, i) in vs.into_iter() {
            if !selected[i] && score_table[i] > threashold {
                ret.push(i);

                let si = if i <= RANGE_THREASHOLD { 0 } else { i - RANGE_THREASHOLD };
                let ei = (i + RANGE_THREASHOLD).min(score_table.len());
                for j in si..ei {
                    selected[j] = true;
                }
            }
        }
        ret.sort();

        ret
    };

    // 自分のマスの右で切るコストを管理
    let mut row_score_table = vec![0f64; image.height + 1];
    row_score_table[0] = 1e10;
    row_score_table[image.height] = 1e10;

    for y in 1..image.height - 1 {
        for x in 0..image.width {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y + 1, x).to64();
            row_score_table[y] += (c1 - c2).abs().horizontal_max();
        }
        // 1画素あたり、最大どれくらいずれてる？という尺度に直す
        row_score_table[y] /= image.width as f64;
    }

    // 自分のマスの上で切るコストを管理
    let mut col_score_table = vec![0f64; image.width + 1];
    col_score_table[0] = 1e10;
    col_score_table[image.width] = 1e10;

    for x in 1..image.width - 1 {
        for y in 0..image.height {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y, x + 1).to64();
            col_score_table[x] += (c1 - c2).abs().horizontal_max();
        }
        col_score_table[x] /= image.height as f64;
    }

    (select_value(&row_score_table), select_value(&col_score_table))
}

fn calculate_divisor_list(value: usize) -> Vec<usize> {
    let mut ret = vec![];
    for i in (2..value).rev() {
        if value % i == 0 {
            ret.push(i);
        }
    }
    ret
}

pub fn solve(problem_id: usize, image: &Image) -> State {
    const POS_THREASHOLD: usize = 50;

    let mut best_state = State::new(image.height, image.width);
    let mut best_score = 1e10; // evaluate(image, &best_state);

    let save_image = |state: &State, counter: &mut usize| {
        let filepath = format!("intermediate_{}_{}.png", problem_id, counter);
        state.save_image(&filepath);
        *counter += 1;
    };

    let mut counter = 0;

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

            save_image(&state, &mut counter);
            if best_score > exact_score {
                best_score = exact_score;
                best_state = state;
            }
        } else {
            eprintln!("cannot solve with edge: row = {}, column = {}", row_list.len(), column_list.len());
        }
    }

    if false {
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
            save_image(&state, &mut counter);
            eprintln!("update: {} -> {}", best_score, exact_score);
            if best_score > exact_score {
                best_score = exact_score;
                best_state = state;
            }
        }
    }

    best_state
}

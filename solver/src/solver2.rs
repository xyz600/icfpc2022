use crate::common_solver::solve_by_divisor;
use common::problem::*;

fn detect_edge(image: &Image, threashold: f64) -> (Vec<usize>, Vec<usize>) {
    let mut row_edge_list = vec![];

    fn filter(num_list: Vec<usize>) -> Vec<usize> {
        let mut ret = vec![];

        let mut index = 0;
        // 連続している値があったら、中央取るとよさそう
        while index < num_list.len() {
            if index == 0 || index == num_list.len() - 1 {
                ret.push(num_list[index]);
                index += 1;
            } else if num_list[index] == 1 || num_list[index] == num_list.last().unwrap() - 1 {
                index += 1;
            } else {
                // 1 と last - 1 はどうせ採択されるので、スキップ

                let first_index = index;
                while index < num_list.len() - 1 && num_list[index] + 1 == num_list[index + 1] {
                    index += 1;
                }
                let last_index = index;
                let selected = (last_index + first_index) / 2;
                ret.push(num_list[selected]);
                index += 1;
            }
        }

        ret
    }

    row_edge_list.push(0);
    for y in 1..image.height - 1 {
        for x in 0..image.width {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y + 1, x).to64();
            if (c1 - c2).abs().horizontal_add() > threashold {
                row_edge_list.push(y);
                break;
            }
        }
    }
    row_edge_list.push(image.height);

    let mut col_edge_list = vec![];
    col_edge_list.push(0);
    for x in 1..image.width - 1 {
        for y in 0..image.height {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y, x + 1).to64();
            if (c1 - c2).abs().horizontal_add() > threashold {
                col_edge_list.push(x);
                break;
            }
        }
    }
    col_edge_list.push(image.width);

    (filter(row_edge_list), filter(col_edge_list))
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
    const POS_THREASHOLD: usize = 40;

    let mut best_state = State::new(image.height, image.width);
    let mut best_score = std::f64::MAX; // evaluate(image, &best_state);

    let save_image = |state: &State| {
        let filepath = format!("intermediate_{}.png", problem_id);
        state.save_image(&filepath);
    };

    {
        // edge 検出して、パターン数が少なければやってみる
        let (row_list, column_list) = detect_edge(image, 30.0);
        if row_list.len() <= POS_THREASHOLD && column_list.len() <= POS_THREASHOLD {
            eprintln!("trying edge based division");
            eprintln!("row {:?}", row_list);
            eprintln!("col {:?}", column_list);
            let state = solve_by_divisor(image, &row_list, &column_list);
            let exact_score = evaluate(image, &state);
            eprintln!("update: {} -> {}", best_score, exact_score);
            if best_score > exact_score {
                best_score = exact_score;
                best_state = state;
                save_image(&best_state);
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
            if best_score > exact_score {
                eprintln!("update: {} -> {}", best_score, exact_score);
                best_score = exact_score;
                best_state = state;
                save_image(&best_state);
            }
        }
    }

    best_state
}

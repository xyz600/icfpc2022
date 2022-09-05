use std::time::Instant;

use crate::common_solver::solve_by_divisor;
use common::{clustering::k_means_clustering, problem::*, random::CachedRandom};

// row_list, column_list
fn extract_edge_from_rect(rect_list: &Vec<Rectangle>, height: usize, width: usize) -> (Vec<usize>, Vec<usize>) {
    let mut row_list = vec![];
    let mut column_list = vec![];
    row_list.push(0);
    row_list.push(height);
    column_list.push(0);
    column_list.push(width);

    for rect in rect_list.iter() {
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

    (row_list, column_list)
}

pub fn solve(problem_id: usize, image: &Image) -> State {
    // for k-means clustering
    let mut color_buffer = vec![];
    for y in 0..image.height {
        for x in 0..image.width {
            color_buffer.push(image.color_of(y, x));
        }
    }
    let color_buffer = color_buffer;

    let mut random = CachedRandom::new(65536, 42);

    const INVALID_ID: usize = std::usize::MAX;

    let mut best_state = State::new(image.height, image.width);
    let mut best_eval = evaluate(image, &best_state);

    for num_color in 2..10 {
        // k-means clustering で色分け
        // なければ終了
        if let Some((assign_table, color_list)) = k_means_clustering(&color_buffer, num_color, &mut random) {
            // 色の代わりに色 id で table 作り直し
            let mut color_number_table = vec![vec![0; image.width]; image.height];
            for y in 0..image.height {
                for x in 0..image.width {
                    let index = y * image.width + x;
                    color_number_table[y][x] = assign_table[index];
                }
            }
            {
                // debug
                let mut clone = image.clone();
                for y in 0..image.height {
                    for x in 0..image.width {
                        let from_index = y * image.width + x;
                        let to_index = (image.height - 1 - y) * image.width + x;
                        clone.buffer[to_index] = color_list[assign_table[from_index]];
                    }
                }
                let filepath = "intermediate.png".to_string();
                clone.save_image(&filepath);
            }

            let mut rectangle_buffer = vec![];

            loop {
                // O(n^3) かけて長方形を探しに行く
                // dp[c][y][x] := 色c, (y, x) から右に何個同じ画素が続いているか
                let mut dp = vec![vec![vec![0; image.width + 1]; image.height + 1]; num_color];
                for c in 0..num_color {
                    for y in 0..image.height {
                        let mut counter = 0;
                        for x in (0..image.width).rev() {
                            if color_number_table[y][x] == c {
                                counter += 1;
                            } else {
                                counter = 0;
                            }
                            dp[c][y][x] = counter;
                        }
                    }
                }

                let mut best_rect = Rectangle::new(0, 0, 1, 1);
                let mut best_color = 0;
                let mut best_size = 0;

                // 長方形を見つける
                for c in 0..num_color {
                    for sy in 0..image.height {
                        for x in 0..image.width {
                            if dp[c][sy][x] == 0 {
                                continue;
                            }
                            let mut cum_right = image.width - 1;
                            for ey in sy..image.height {
                                if dp[c][ey][x] == 0 {
                                    break;
                                }
                                let right = x + dp[c][ey][x] - 1;
                                cum_right = cum_right.min(right);

                                let height = ey + 1 - sy;
                                let width = cum_right + 1 - x;
                                let size = height * width;
                                if best_size < size {
                                    best_size = size;
                                    best_rect = Rectangle::new(sy, x, height, width);
                                    best_color = c;
                                }
                            }
                        }
                    }
                }
                eprintln!("c = {}. {:?}", best_color, best_rect);

                // 長方形の color_number_table を INVALID で上書き
                for y in best_rect.bottom()..=best_rect.top() {
                    for x in best_rect.left()..=best_rect.right() {
                        assert_eq!(color_number_table[y][x], best_color);
                        color_number_table[y][x] = INVALID_ID;
                    }
                }
                rectangle_buffer.push(best_rect);

                let start_time = Instant::now();
                let (row_list, column_list) = extract_edge_from_rect(&rectangle_buffer, image.height, image.width);
                let elapsed = (Instant::now() - start_time).as_secs();

                eprintln!("row_list: {:?}", row_list);
                eprintln!("column_list: {:?}", column_list);

                let state = solve_by_divisor(image, &row_list, &column_list);
                let eval = evaluate(image, &state);

                eprintln!("color num: {}, rect_num: {}, eval = {}", num_color, rectangle_buffer.len(), eval);

                if best_eval > eval {
                    best_eval = eval;
                    best_state = state;
                    StateWithScore {
                        score: best_eval,
                        state: best_state.clone(),
                    }
                    .save_if_global_best(problem_id);
                }

                if elapsed > 20 {
                    break;
                }
            }
        } else {
            break;
        }
    }
    best_state
}

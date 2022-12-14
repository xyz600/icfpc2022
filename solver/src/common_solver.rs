use common::cumulative_sum::RangeColorMedianCalculator;
use common::problem::*;
use std::collections::VecDeque;

// ブロック番号が確定しないので、メモ用途だけのコマンド
#[derive(Clone, Copy, Debug)]
enum SimpleCommand {
    // xi
    VerticalSplit(usize),
    // yi
    HorizontalSplit(usize),
    // yi, xi
    PointSplit(usize, usize),
    Color(Color8),
}

/// 縦分割、横分割、十字分割を試して、色の塗り方は愚直に色々試す
/// table_height, table_width それぞれ 50 位が限界？
/// 端点に 0, と image.height / image.width を入れてください…
pub fn solve_by_divisor(image: &Image, row_list: &Vec<usize>, column_list: &Vec<usize>) -> State {
    const INF_COST: f64 = std::f64::MAX;

    for yi in 0..row_list.len() - 1 {
        assert!(row_list[yi] < row_list[yi + 1]);
    }
    for xi in 0..column_list.len() - 1 {
        assert!(column_list[xi] < column_list[xi + 1]);
    }
    assert!(row_list[0] == 0);
    assert!(*row_list.last().unwrap() == image.height);
    assert!(column_list[0] == 0);
    assert!(*column_list.last().unwrap() == image.width);

    let table_height = row_list.len();
    let table_width = column_list.len();

    // DP[y1][x1][y2][x2] := [row_list[y1], row_list[y2]), [col_list[x1], col_list[x2]) の領域を生コスト最小で塗る方法
    let mut dp = vec![vec![vec![vec![INF_COST; table_width]; table_height]; table_width]; table_height];
    // 行動復元用のコマンド
    let mut restore_table = vec![vec![vec![vec![None; table_width]; table_height]; table_width]; table_height];

    let median_calculator = RangeColorMedianCalculator::new(image);

    fn inner(
        image: &Image,
        median_calculator: &RangeColorMedianCalculator,
        height: usize,
        width: usize,
        dp: &mut Vec<Vec<Vec<Vec<f64>>>>,
        // (このタイルが行うべきコマンド, 子供の何番目の色を塗るか, 何色で塗るか)
        // SimpleCommand::Color だった場合は意味のないデータになる
        restore_table: &mut Vec<Vec<Vec<Vec<Option<(SimpleCommand, usize, Color8)>>>>>,
        row_list: &Vec<usize>,
        column_list: &Vec<usize>,
        y1: usize,
        x1: usize,
        y2: usize,
        x2: usize,
    ) {
        assert!(y1 <= y2);
        assert!(x1 <= x2);

        let canvas_size = height * width;

        let calculate_block_size = |y1: usize, x1: usize, y2: usize, x2: usize| -> usize { (row_list[y2] - row_list[y1]) * (column_list[x2] - column_list[x1]) };

        let calculate_color_cost = |y1: usize, x1: usize, y2: usize, x2: usize| -> (f64, Color8) {
            let sy = row_list[y1];
            let sx = column_list[x1];
            let ey = row_list[y2];
            let ex = column_list[x2];

            // 画素値が大体同じ値だと仮定すると、sqrt(n) で割る位がちょうどよさそう
            let color = median_calculator.median(sy, sx, ey, ex);
            let color64 = color.to64();
            let mut rmse_sum = 0.0;
            for y in sy..ey {
                for x in sx..ex {
                    rmse_sum += (image.color_of(y, x).to64() - color64).square().horizontal_add().sqrt();
                }
            }

            let block_size = calculate_block_size(y1, x1, y2, x2);
            let command_cost = COLOR_COST * canvas_size as f64 / block_size as f64;

            // FIXME: 後で直した方がいいかも？
            (rmse_sum * ALPHA + command_cost, color)
        };

        let calculate_line_cut_cost = |y1: usize, x1: usize, y2: usize, x2: usize| -> f64 {
            let canvas_size = height * width;
            let block_size = calculate_block_size(y1, x1, y2, x2);
            LINE_CUT_COST * canvas_size as f64 / block_size as f64
        };

        let calculate_point_cut_cost = |y1: usize, x1: usize, y2: usize, x2: usize| -> f64 {
            let canvas_size = height * width;
            let block_size = calculate_block_size(y1, x1, y2, x2);
            POINT_CUT_COST * canvas_size as f64 / block_size as f64
        };

        // 答えが埋まっている場合はそれを返す
        if dp[y1][x1][y2][x2] != INF_COST {
            return;
        }

        // そのまま色を塗るコストを計算
        let (color_cost, color) = calculate_color_cost(y1, x1, y2, x2);
        if dp[y1][x1][y2][x2] > color_cost {
            dp[y1][x1][y2][x2] = color_cost;
            // 後ろ2つは意味ない
            restore_table[y1][x1][y2][x2] = Some((SimpleCommand::Color(color), 0, color));
        }
        let self_block_size = calculate_block_size(y1, x1, y2, x2);

        // 横分割して再帰
        for yi in y1 + 1..y2 {
            inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, y1, x1, yi, x2);
            inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, yi, x1, y2, x2);

            // 色の塗り方を工夫することで、自分の色を塗ってから最大コストの色塗りを1つ回避できる
            let block_size1 = calculate_block_size(y1, x1, yi, x2);
            let block_size2 = calculate_block_size(yi, x1, y2, x2);
            let max_color_cost = COLOR_COST * canvas_size as f64 / block_size1.min(block_size2) as f64;
            let self_color_cost = COLOR_COST * canvas_size as f64 / self_block_size as f64;

            let vert_cost = dp[y1][x1][yi][x2] + dp[yi][x1][y2][x2] + calculate_line_cut_cost(y1, x1, y2, x2) + self_color_cost - max_color_cost;
            if dp[y1][x1][y2][x2] > vert_cost {
                dp[y1][x1][y2][x2] = vert_cost;

                let (child_index, child_color) = if block_size1 < block_size2 {
                    (0, restore_table[y1][x1][yi][x2].unwrap().2)
                } else {
                    (1, restore_table[yi][x1][y2][x2].unwrap().2)
                };
                restore_table[y1][x1][y2][x2] = Some((SimpleCommand::HorizontalSplit(yi), child_index, child_color));
            }
        }

        // 縦分割して再帰
        for xi in x1 + 1..x2 {
            inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, y1, x1, y2, xi);
            inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, y1, xi, y2, x2);

            // 色の塗り方を工夫することで、自分の色を塗ってから最大コストの色塗りを1つ回避できる
            let block_size1 = calculate_block_size(y1, x1, y2, xi);
            let block_size2 = calculate_block_size(y1, xi, y2, x2);
            let max_color_cost = COLOR_COST * canvas_size as f64 / block_size1.min(block_size2) as f64;
            let self_color_cost = COLOR_COST * canvas_size as f64 / self_block_size as f64;

            let hor_cost = dp[y1][x1][y2][xi] + dp[y1][xi][y2][x2] + calculate_line_cut_cost(y1, x1, y2, x2) + self_color_cost - max_color_cost;
            if dp[y1][x1][y2][x2] > hor_cost {
                dp[y1][x1][y2][x2] = hor_cost;

                let (child_index, child_color) = if block_size1 < block_size2 {
                    (0, restore_table[y1][x1][y2][xi].unwrap().2)
                } else {
                    (1, restore_table[y1][xi][y2][x2].unwrap().2)
                };
                restore_table[y1][x1][y2][x2] = Some((SimpleCommand::VerticalSplit(xi), child_index, child_color));
            }
        }

        // 点分割して再帰
        for yi in y1 + 1..y2 {
            for xi in x1 + 1..x2 {
                inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, y1, x1, yi, xi);
                inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, y1, xi, yi, x2);
                inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, yi, xi, y2, x2);
                inner(image, median_calculator, height, width, dp, restore_table, row_list, column_list, yi, x1, y2, xi);

                // 色の塗り方を工夫することで、自分の色を塗ってから最大コストの色塗りを1つ回避できる
                let block_size_list = vec![
                    calculate_block_size(y1, x1, yi, xi),
                    calculate_block_size(y1, xi, yi, x2),
                    calculate_block_size(yi, xi, y2, x2),
                    calculate_block_size(yi, x1, y2, xi),
                ];
                let min_block_size = *block_size_list.iter().min().unwrap();
                let max_color_cost = COLOR_COST * canvas_size as f64 / min_block_size as f64;
                let self_color_cost = COLOR_COST * canvas_size as f64 / self_block_size as f64;

                let point_cost = dp[y1][x1][yi][xi] + dp[y1][xi][yi][x2] + dp[yi][xi][y2][x2] + dp[yi][x1][y2][xi] + calculate_point_cut_cost(y1, x1, y2, x2) + self_color_cost - max_color_cost;

                if dp[y1][x1][y2][x2] > point_cost {
                    dp[y1][x1][y2][x2] = point_cost;

                    let mut child_index = 0;
                    let mut child_color = Color8::new(0, 0, 0, 0);
                    let color_list = vec![
                        restore_table[y1][x1][yi][xi].unwrap().2,
                        restore_table[y1][xi][yi][x2].unwrap().2,
                        restore_table[yi][xi][y2][x2].unwrap().2,
                        restore_table[yi][x1][y2][xi].unwrap().2,
                    ];
                    for i in 0..4 {
                        if block_size_list[0] == min_block_size {
                            child_index = i;
                            child_color = color_list[i];
                            break;
                        }
                    }
                    restore_table[y1][x1][y2][x2] = Some((SimpleCommand::PointSplit(yi, xi), child_index, child_color));
                }
            }
        }
    }

    inner(
        image,
        &median_calculator,
        image.height,
        image.width,
        &mut dp,
        &mut restore_table,
        row_list,
        column_list,
        0,
        0,
        row_list.len() - 1,
        column_list.len() - 1,
    );

    // コマンドを復元
    let mut state = State::new(image.height, image.width);
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, row_list.len() - 1, column_list.len() - 1, 0, true));

    while let Some((y1, x1, y2, x2, block_index, color_self)) = queue.pop_front() {
        let (cmd, child_index, child_color) = restore_table[y1][x1][y2][x2].unwrap();
        match cmd {
            SimpleCommand::VerticalSplit(xi) => {
                let x = column_list[xi];
                let child_block_index = state.block_list.len();
                let cmd = Command::VerticalSplit(block_index, x);
                if color_self {
                    state.apply(Command::Color(block_index, child_color));
                }
                state.apply(cmd);
                queue.push_back((y1, x1, y2, xi, child_block_index, child_index != 0));
                queue.push_back((y1, xi, y2, x2, child_block_index + 1, child_index != 1));
            }
            SimpleCommand::HorizontalSplit(yi) => {
                let y = row_list[yi];
                let child_block_index = state.block_list.len();
                let cmd = Command::HorizontalSplit(block_index, y);
                if color_self {
                    state.apply(Command::Color(block_index, child_color));
                }
                state.apply(cmd);
                queue.push_back((y1, x1, yi, x2, child_block_index, child_index != 0));
                queue.push_back((yi, x1, y2, x2, child_block_index + 1, child_index != 1));
            }
            SimpleCommand::PointSplit(yi, xi) => {
                let y = row_list[yi];
                let x = column_list[xi];
                let cmd = Command::PointSplit(block_index, Pos::new(y, x));
                let child_block_index = state.block_list.len();
                if color_self {
                    state.apply(Command::Color(block_index, child_color));
                }
                state.apply(cmd);
                queue.push_back((y1, x1, yi, xi, child_block_index, child_index != 0));
                queue.push_back((y1, xi, yi, x2, child_block_index + 1, child_index != 1));
                queue.push_back((yi, xi, y2, x2, child_block_index + 2, child_index != 2));
                queue.push_back((yi, x1, y2, xi, child_block_index + 3, child_index != 3));
            }
            SimpleCommand::Color(color) => {
                if color_self {
                    state.apply(Command::Color(block_index, color));
                }
            }
        }
    }
    state
}

// 画素値で threashold 以上切れていそうな部分を見つけ、優先的に配置
pub fn detect_edge(image: &Image, threashold: f64) -> (Vec<usize>, Vec<usize>) {
    let select_value = |score_table: &Vec<f64>| -> Vec<usize> {
        const RANGE_THREASHOLD: usize = 3;

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
            row_score_table[y] = row_score_table[y].max((c1 - c2).abs().horizontal_max());
        }
    }

    // 自分のマスの上で切るコストを管理
    let mut col_score_table = vec![0f64; image.width + 1];
    col_score_table[0] = 1e10;
    col_score_table[image.width] = 1e10;

    for x in 1..image.width - 1 {
        for y in 0..image.height {
            let c1 = image.color_of(y, x).to64();
            let c2 = image.color_of(y, x + 1).to64();
            col_score_table[x] = col_score_table[x].max((c1 - c2).abs().horizontal_max());
        }
    }

    (select_value(&row_score_table), select_value(&col_score_table))
}

pub fn calculate_divisor_list(value: usize) -> Vec<usize> {
    let mut ret = vec![];
    for i in (2..value).rev() {
        if value % i == 0 {
            ret.push(i);
        }
    }
    ret
}

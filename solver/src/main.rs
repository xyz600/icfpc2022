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

    // 雑に間引いて、これまで切った部分は
    let (row_edge_list, col_edge_list) = detect_edge(image, 30.0);

    eprintln!("{:?}", row_edge_list);
    eprintln!("{:?}", col_edge_list);

    loop {
        let rmse_cumulator = CumulativeRMSESum::new(image, &state);

        // 貪欲法
        // 以下の選択肢で一番良さそうなものを選択
        // 1. 横線1本 + 2色
        // 2. 縦線1本 + 2色
        // 3. 点1つ + 4色
        // 4. 横線2本 + 3色
        // 5. 縦線2本 + 3色

        break;
    }

    state
}

fn main() {
    let app = App::new("xyzsolver")
        .version("1")
        .author("xyz600")
        .about("problem solver for icfpc 2022")
        .arg(
            Arg::with_name("input-filepath")
                .help("input png filepath")
                .short('i')
                .long("input-filepath")
                .required(true)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let input_filepath = matches.value_of("input-filepath").unwrap();
    let image = Image::new(input_filepath);

    let final_state = solve(&image);
    final_state.print_output();
}

use clap::{App, Arg};
use common::*;

fn solve(image: &Image) -> State {
    let mut state = State::new(image.height, image.width);

    // 貪欲法

    // エッジ検出

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

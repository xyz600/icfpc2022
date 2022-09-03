use clap::{App, Arg};
use common::*;

fn solve(image: &Image) -> State {
    let mut state = State::new(image.height, image.width);

    state.apply(Command::PointSplit(0, Pos::new(100, 100)));
    for i in 1..5 {
        let parent_color = state.block_list[0].color;
        state.apply(Command::Color(
            i,
            parent_color,
            Color::new(32 * i as u8, 32 * i as u8, 32 * i as u8, 32 * i as u8),
        ));
    }

    state.apply(Command::HorizontalSplit(4, 300));
    state.apply(Command::Color(
        6,
        state.block_list[6].color,
        Color::new(0, 0, 128, 192),
    ));

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

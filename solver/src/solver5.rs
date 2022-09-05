use crate::{solver2, solver6};
use common::problem::*;
use std::path::Path;

// 全て merge して、 solver2::solve と同じことをします
pub fn solve(problem_id: usize, image: &Image) -> State {
    let str_path = format!("dataset/{}.initial.json", problem_id);
    let config_path = Path::new(&str_path);
    let config = common::config_loader::TwinImageConfig::load(config_path);

    let mut state = State::create_with_config(&config);

    let unit = state.block_list.last().unwrap().rect.height;
    let dim = image.height / unit;

    let mut block_index_table = vec![vec![0; dim]; dim];
    for block in state.block_list.iter() {
        let iy = block.rect.bottom() / unit;
        let ix = block.rect.left() / unit;
        block_index_table[iy][ix] = block.index_of;
    }

    let mut row_block_index = vec![0; dim];
    for yi in 0..dim {
        let mut block_index_from = block_index_table[yi][0];
        for xi in 1..dim {
            state.apply(Command::Merge(block_index_from, block_index_table[yi][xi]));
            block_index_from = state.block_list.len() - 1;
        }
        row_block_index[yi] = state.block_list.len() - 1;
    }
    {
        let mut block_index_from = row_block_index[0];
        for yi in 1..dim {
            state.apply(Command::Merge(block_index_from, row_block_index[yi]));
            block_index_from = state.block_list.len() - 1;
        }
    }
    // 初期状態に戻す
    state.apply(Command::Color(state.block_list.len() - 1, Color8::new(255, 255, 255, 255)));

    // almost all solver2
    // ただし、solve させた後コマンドの移し替えをする

    let apply_prestate = |s: State| -> State {
        let mut clone = state.clone();
        let offset = clone.block_list.len() - 1;
        for cmd in s.get_command_list().into_iter() {
            match cmd {
                Command::HorizontalSplit(block_index, y) => clone.apply(Command::HorizontalSplit(block_index + offset, y)),
                Command::VerticalSplit(block_index, x) => clone.apply(Command::VerticalSplit(block_index + offset, x)),
                Command::PointSplit(block_index, pos) => clone.apply(Command::PointSplit(block_index + offset, pos)),
                Command::Color(block_index, color) => clone.apply(Command::Color(block_index + offset, color)),
                Command::Swap(block_index1, block_index2) => clone.apply(Command::Swap(block_index1 + offset, block_index2 + offset)),
                Command::Merge(block_index1, block_index2) => clone.apply(Command::Merge(block_index1 + offset, block_index2 + offset)),
            }
        }
        clone
    };

    let pre_state = solver2::solve(problem_id, image);
    apply_prestate(pre_state)
}

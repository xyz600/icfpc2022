use png::{ColorType, Decoder};
use std::fs::File;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

pub struct Image {
    pub height: usize,
    pub width: usize,
    pub buffer: Vec<Color>,
}

impl Image {
    pub fn new(filepath: String) -> Image {
        let decoder = png::Decoder::new(File::open(filepath.as_str()).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut raw_buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut raw_buffer).unwrap();
        assert!(info.color_type == ColorType::Rgba);

        //
        let height = info.height as usize;
        let width = info.width as usize;
        let mut buffer = vec![];
        for i in 0..height * width {
            let r = raw_buffer[4 * i];
            let g = raw_buffer[4 * i + 1];
            let b = raw_buffer[4 * i + 2];
            let a = raw_buffer[4 * i + 3];
            buffer.push(Color::new(r, g, b, a));
        }
        Image {
            height,
            width,
            buffer,
        }
    }

    pub fn color(&self, pos: &Pos) -> Color {
        self.buffer[pos.y * self.width + pos.x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    pub fn new(y: usize, x: usize) -> Pos {
        Pos { y, x }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rectangle {
    pub bottom_left: Pos,
    pub height: usize,
    pub width: usize,
}

impl Rectangle {
    pub fn new(bottom: usize, left: usize, height: usize, width: usize) -> Rectangle {
        Rectangle {
            bottom_left: Pos { y: bottom, x: left },
            height,
            width,
        }
    }

    pub fn right(&self) -> usize {
        self.bottom_left.x + self.width - 1
    }
    pub fn left(&self) -> usize {
        self.bottom_left.x
    }
    pub fn bottom(&self) -> usize {
        self.bottom_left.y
    }
    pub fn top(&self) -> usize {
        self.bottom_left.y + self.height - 1
    }

    /// 辺上に存在するのは ok
    pub fn contains(&self, pos: &Pos) -> bool {
        self.left() <= pos.x
            && pos.x <= self.right()
            && self.bottom() <= pos.y
            && pos.y <= self.top()
    }

    /// 辺上にも存在しない
    pub fn is_internal(&self, pos: &Pos) -> bool {
        self.left() < pos.x && pos.x < self.right() && self.bottom() < pos.y && pos.y < self.top()
    }

    /// x is exclusive for left
    pub fn vertical_split(&self, x: usize) -> (Rectangle, Rectangle) {
        assert!(self.left() < x && x < self.right());
        let left = Rectangle::new(self.bottom(), self.left(), self.height, x - self.left());
        let right = Rectangle::new(self.bottom(), x, self.height, self.right() + 1 - x);
        (left, right)
    }

    /// y is explicit for top
    pub fn horizontal_split(&self, y: usize) -> (Rectangle, Rectangle) {
        assert!(self.bottom() < y && y < self.top());
        let bottom = Rectangle::new(self.bottom(), self.left(), y - self.bottom(), self.width);
        let top = Rectangle::new(y, self.left(), self.top() + 1 - y, self.width);
        (bottom, top)
    }

    pub fn point_split(&self, p: &Pos) -> (Rectangle, Rectangle, Rectangle, Rectangle) {
        assert!(self.is_internal(p));
        let (left, right) = self.vertical_split(p.x);
        let (bottom_left, top_left) = left.horizontal_split(p.y);
        let (bottom_right, top_right) = right.horizontal_split(p.y);

        (bottom_left, bottom_right, top_right, top_left)
    }
}

/// FIXME: merge 操作を特別視する. enum 用意する？
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block {
    pub rect: Rectangle,
    pub color: Color,
    // 操作元のブロック index
    pub parent: Option<usize>,
    // 出力用
    pub id: usize,
    // 自分が操作の末端か
    pub is_child: bool,
    // 管理用配列の index
    index_of: usize,
}

impl Block {
    pub fn vertical_split(&self, x: usize, child_start_index: usize) -> (Block, Block) {
        let (left_rect, right_rect) = self.rect.vertical_split(x);
        let left_block = Block {
            rect: left_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 0,
            is_child: true,
            index_of: child_start_index,
        };
        let right_block = Block {
            rect: right_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 1,
            is_child: true,
            index_of: child_start_index + 1,
        };
        (left_block, right_block)
    }

    pub fn horizontal_split(&self, y: usize, child_start_index: usize) -> (Block, Block) {
        let (bottom_rect, top_rect) = self.rect.horizontal_split(y);
        let bottom_block = Block {
            rect: bottom_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 0,
            is_child: true,
            index_of: child_start_index,
        };
        let top_block = Block {
            rect: top_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 1,
            is_child: true,
            index_of: child_start_index + 1,
        };
        (bottom_block, top_block)
    }

    pub fn point_split(&self, pos: &Pos, child_start_index: usize) -> (Block, Block, Block, Block) {
        let (bl_rect, br_rect, tr_rect, tl_rect) = self.rect.point_split(pos);
        let bl_block = Block {
            rect: bl_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 0,
            is_child: true,
            index_of: child_start_index,
        };
        let br_block = Block {
            rect: br_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 1,
            is_child: true,
            index_of: child_start_index + 1,
        };
        let tr_block = Block {
            rect: tr_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 2,
            is_child: true,
            index_of: child_start_index + 2,
        };
        let tl_block = Block {
            rect: tl_rect,
            color: self.color,
            parent: Some(self.index_of),
            id: 3,
            is_child: true,
            index_of: child_start_index + 3,
        };
        (bl_block, br_block, tr_block, tl_block)
    }
}

/// 最終的に出力する内容に関わるもの
/// FIXME: color の prev_color を消す
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Command {
    // block_idx, y
    HorizontalSplit(usize, usize),
    // block_idx, x
    VerticalSplit(usize, usize),
    // block_idx, (x, y)
    PointSplit(usize, Pos),
    // block_index, prev_color, color
    Color(usize, Color, Color),
}

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub block_list: Vec<Block>,
    pub command_list: Vec<Command>,
    pub max_block_id: usize,
}

impl State {
    pub fn new(height: usize, width: usize) -> State {
        let init_block = Block {
            rect: Rectangle {
                bottom_left: Pos { y: 0, x: 0 },
                height,
                width,
            },
            color: Color::default(),
            parent: None,
            id: 0,
            is_child: true,
            index_of: 0,
        };
        State {
            block_list: vec![init_block],
            command_list: vec![],
            max_block_id: 0,
        }
    }

    pub fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::HorizontalSplit(block_index, y) => self.horizontal_split(block_index, y),
            Command::VerticalSplit(block_index, x) => self.vertical_split(block_index, x),
            Command::PointSplit(block_index, pos) => self.point_cut(block_index, &pos),
            Command::Color(block_index, prev_color, color) => {
                self.color(block_index, &prev_color, &color)
            }
        }
        self.command_list.push(cmd);
    }

    pub fn undo(&mut self) {
        assert!(!self.command_list.is_empty());
        match *self.command_list.last().unwrap() {
            Command::HorizontalSplit(block_index, _) | Command::VerticalSplit(block_index, _) => {
                for _ in 0..2 {
                    assert!(self.block_list.last().unwrap().parent.unwrap() == block_index);
                    self.block_list.pop();
                }
                assert!(!self.block_list[block_index].is_child);
                self.block_list[block_index].is_child = true;
            }
            Command::PointSplit(block_index, _) => {
                for _ in 0..4 {
                    assert!(self.block_list.last().unwrap().parent.unwrap() == block_index);
                    self.block_list.pop();
                }
                assert!(!self.block_list[block_index].is_child);
                self.block_list[block_index].is_child = true;
            }
            Command::Color(block_index, prev_color, _) => {
                assert!(self.block_list[block_index].is_child);
                self.block_list[block_index].color = prev_color;
            }
        }
        self.command_list.pop();
    }

    pub fn color(&mut self, block_index: usize, prev_color: &Color, color: &Color) {
        assert!(block_index < self.block_list.len());
        assert!(self.block_list[block_index].color == *prev_color);
        self.block_list[block_index].color = *color;
    }

    pub fn horizontal_split(&mut self, block_index: usize, y: usize) {
        assert!(block_index < self.block_list.len());
        let len = self.block_list.len();
        let mut parent_block = &mut self.block_list[block_index];
        assert!(parent_block.index_of == block_index);
        assert!(parent_block.rect.bottom() < y && y < parent_block.rect.top());
        parent_block.is_child = false;

        let (bottom_block, top_block) = parent_block.horizontal_split(y, len);
        self.block_list.push(bottom_block);
        self.block_list.push(top_block);
    }

    pub fn vertical_split(&mut self, block_index: usize, x: usize) {
        assert!(block_index < self.block_list.len());
        let len = self.block_list.len();
        let mut parent_block = &mut self.block_list[block_index];
        assert!(parent_block.index_of == block_index);
        assert!(parent_block.rect.left() < x && x < parent_block.rect.right());
        parent_block.is_child = false;

        let (left_block, right_block) = parent_block.vertical_split(x, len);
        self.block_list.push(left_block);
        self.block_list.push(right_block);
    }

    pub fn point_cut(&mut self, block_index: usize, pos: &Pos) {
        assert!(block_index < self.block_list.len());
        let len = self.block_list.len();
        let mut parent_block = &mut self.block_list[block_index];
        assert!(parent_block.index_of == block_index);
        assert!(parent_block.rect.is_internal(pos));
        assert!(parent_block.is_child);
        parent_block.is_child = false;

        let (bl, br, ur, ul) = parent_block.point_split(pos, len);
        for block in [bl, br, ur, ul] {
            self.block_list.push(block);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contain() {
        let rect = Rectangle::new(5, 5, 10, 10);
        let p1 = Pos::new(10, 10);
        assert!(rect.contains(&p1));

        let p2 = Pos::new(14, 10);
        assert!(rect.contains(&p2));

        let p3 = Pos::new(15, 10);
        assert!(!rect.contains(&p3));
    }

    #[test]
    fn test_rect_split() {
        let rect = Rectangle::new(5, 5, 10, 10);
        let (bl, br, tr, tl) = rect.point_split(&Pos::new(9, 9));

        let expected_bl = Rectangle::new(5, 5, 4, 4);
        assert_eq!(bl, expected_bl);

        let expected_br = Rectangle::new(5, 9, 4, 6);
        assert_eq!(br, expected_br);

        let expected_tr = Rectangle::new(9, 9, 6, 6);
        assert_eq!(tr, expected_tr);

        let expected_tl = Rectangle::new(9, 5, 6, 4);
        assert_eq!(tl, expected_tl);
    }

    #[test]
    fn test_state_undo() {
        let mut state = State::new(400, 400);
        state.apply(Command::PointSplit(0, Pos::new(200, 200)));
        assert_eq!(state.block_list.len(), 5);

        let mut clone = state.clone();
        clone.apply(Command::HorizontalSplit(1, 100));
        assert_eq!(clone.block_list.len(), 7);
        clone.undo();
        assert_eq!(state, clone);

        clone.apply(Command::VerticalSplit(1, 100));
        assert_eq!(clone.block_list.len(), 7);
        clone.undo();
        assert_eq!(state, clone);

        clone.apply(Command::PointSplit(1, Pos::new(50, 50)));
        assert_eq!(clone.block_list.len(), 9);
        clone.undo();
        assert_eq!(state, clone);

        clone.apply(Command::Color(
            1,
            clone.block_list[1].color,
            Color::new(128, 128, 128, 128),
        ));
        assert_eq!(clone.block_list.len(), 5);
        clone.undo();
        assert_eq!(state, clone);
    }
}

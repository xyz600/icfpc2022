use png::ColorType;
use std::{
    fs::File,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Color<T> {
        Color { r, g, b, a }
    }
}

// 生値保存用
type Color8 = Color<u8>;
// 計算用
type Color64 = Color<f64>;

impl Add<Color64> for Color64 {
    type Output = Color64;

    fn add(self, rhs: Color64) -> Self::Output {
        Color64 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl AddAssign<Color64> for Color64 {
    fn add_assign(&mut self, rhs: Color64) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Add<f64> for Color64 {
    type Output = Color64;

    fn add(self, rhs: f64) -> Self::Output {
        Color64 {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a + rhs,
        }
    }
}

impl AddAssign<f64> for Color64 {
    fn add_assign(&mut self, rhs: f64) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
    }
}

impl Sub<f64> for Color64 {
    type Output = Color64;

    fn sub(self, rhs: f64) -> Self::Output {
        Color64 {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
            a: self.a - rhs,
        }
    }
}

impl Sub<Color64> for Color64 {
    type Output = Color64;

    fn sub(self, rhs: Color64) -> Self::Output {
        Color64 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl SubAssign<f64> for Color64 {
    fn sub_assign(&mut self, rhs: f64) {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
    }
}

impl SubAssign<Color64> for Color64 {
    fn sub_assign(&mut self, rhs: Color64) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl Mul<f64> for Color64 {
    type Output = Color64;

    fn mul(self, rhs: f64) -> Self::Output {
        Color64 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl MulAssign<f64> for Color64 {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Div<f64> for Color64 {
    type Output = Color64;

    fn div(self, rhs: f64) -> Self::Output {
        Color64 {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl DivAssign<f64> for Color64 {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}

impl Color64 {
    pub fn to8(&self) -> Color<u8> {
        Color {
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
            a: self.a as u8,
        }
    }

    pub fn square(&self) -> Color64 {
        Color {
            r: self.r * self.r,
            g: self.g * self.g,
            b: self.b * self.b,
            a: self.a * self.a,
        }
    }

    pub fn round(&self) -> Color64 {
        Color {
            r: self.r.round(),
            g: self.g.round(),
            b: self.b.round(),
            a: self.a.round(),
        }
    }

    pub fn horizontal_add(&self) -> f64 {
        self.r + self.g + self.b + self.a
    }
}

impl Color8 {
    pub fn to64(&self) -> Color64 {
        Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}

impl PartialEq for Color8 {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl Eq for Color8 {}

impl Default for Color<u8> {
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
    pub buffer: Vec<Color8>,
}

impl Image {
    pub fn new(filepath: &str) -> Image {
        let decoder = png::Decoder::new(File::open(filepath).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut raw_buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut raw_buffer).unwrap();
        assert!(info.color_type == ColorType::Rgba);

        //
        let height = info.height as usize;
        let width = info.width as usize;
        let mut buffer = vec![Color8::default(); height * width];
        for y in (0..height).rev() {
            for x in 0..width {
                let i = y * width + x;
                let r = raw_buffer[4 * i];
                let g = raw_buffer[4 * i + 1];
                let b = raw_buffer[4 * i + 2];
                let a = raw_buffer[4 * i + 3];
                buffer[i] = Color::new(r, g, b, a);
            }
        }
        Image {
            height,
            width,
            buffer,
        }
    }

    pub fn size(&self) -> usize {
        self.height * self.width
    }

    pub fn color_of_pos(&self, pos: &Pos) -> Color8 {
        self.buffer[pos.y * self.width + pos.x]
    }

    pub fn color_of(&self, y: usize, x: usize) -> Color8 {
        self.buffer[y * self.width + x]
    }

    pub fn mean_color(&self, rect: &Rectangle) -> Color8 {
        let mut sum = Color64::new(0f64, 0f64, 0f64, 0f64);
        for y in rect.bottom()..=rect.top() {
            for x in rect.left()..=rect.right() {
                sum += self.color_of(y, x).to64();
            }
        }
        (sum / rect.size() as f64).to8()
    }

    pub fn rmse(&self, rect: &Rectangle, target_color: &Color8) -> f64 {
        let target_color = target_color.to64();

        let mut sum = 0f64;
        for y in rect.bottom()..=rect.top() {
            for x in rect.left()..=rect.right() {
                let color = self.color_of(y, x).to64();
                sum += (color - target_color).square().horizontal_add().sqrt();
            }
        }
        sum
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

    pub fn size(&self) -> usize {
        self.height * self.width
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
    pub color: Color8,
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
    // block_idx, prev_color, color
    Color(usize, Color8, Color8),
}

impl Command {
    pub fn base_cost(&self) -> usize {
        match *self {
            Command::HorizontalSplit(_, _) => 7,
            Command::VerticalSplit(_, _) => 7,
            Command::PointSplit(_, _) => 10,
            Command::Color(_, _, _) => 5,
        }
    }

    pub fn block_index(&self) -> usize {
        match *self {
            Command::HorizontalSplit(block_index, _) => block_index,
            Command::VerticalSplit(block_index, _) => block_index,
            Command::PointSplit(block_index, _) => block_index,
            Command::Color(block_index, _, _) => block_index,
        }
    }
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
            color: Color8::default(),
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

    fn color(&mut self, block_index: usize, prev_color: &Color8, color: &Color8) {
        assert!(block_index < self.block_list.len());
        assert!(self.block_list[block_index].color == *prev_color);
        self.block_list[block_index].color = *color;
    }

    fn horizontal_split(&mut self, block_index: usize, y: usize) {
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

    fn vertical_split(&mut self, block_index: usize, x: usize) {
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

    fn point_cut(&mut self, block_index: usize, pos: &Pos) {
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

    pub fn print_output(&self) {
        let restore_id_sequence = |block_index: usize| -> String {
            let mut id_list = vec![];
            let mut index = block_index;
            loop {
                id_list.push(self.block_list[index].id);
                if let Some(parent_block_index) = self.block_list[index].parent {
                    index = parent_block_index;
                } else {
                    break;
                }
            }
            id_list.reverse();
            id_list
                .into_iter()
                .map(|v: usize| v.to_string())
                .collect::<Vec<_>>()
                .join(".")
        };

        for cmd in self.command_list.iter() {
            match *cmd {
                Command::HorizontalSplit(block_index, y) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("cut [{}] [y] [{}]", block_id, y);
                }
                Command::VerticalSplit(block_index, x) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("cut [{}] [x] [{}]", block_id, x);
                }
                Command::PointSplit(block_index, pos) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("cut [{}] [{}, {}]", block_id, pos.x, pos.y);
                }
                Command::Color(block_index, _, color) => {
                    let block_id = restore_id_sequence(block_index);
                    println!(
                        "color [{}] [{}, {}, {}, {}] ",
                        block_id, color.r, color.g, color.b, color.a
                    );
                }
            }
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

pub fn evaluate(state: &State, image: &Image) -> f64 {
    let mut pixel_cost = 0f64;
    for block in state.block_list.iter() {
        if block.is_child {
            pixel_cost += image.rmse(&block.rect, &block.color);
        }
    }

    let mut command_cost = 0;
    for cmd in state.command_list.iter() {
        let base_cost = cmd.base_cost();
        let block_index = cmd.block_index();
        command_cost += image.size() / state.block_list[block_index].rect.size() * base_cost;
    }

    pixel_cost.sqrt() + command_cost as f64
}

pub const LINE_CUT_COST: f64 = 7.0;
pub const POINT_CUT_COST: f64 = 10.0;
pub const COLOR_COST: f64 = 5.0;
pub const SWAP_COST: f64 = 3.0;
pub const MERGE_COST: f64 = 1.0;
pub const ALPHA: f64 = 0.005;

use png::ColorType;
use std::{
    fs::File,
    io::BufWriter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    path::Path,
};

use crate::config_loader;

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
pub type Color8 = Color<u8>;
// 計算用
pub type Color64 = Color<f64>;

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

impl Mul<Color64> for Color64 {
    type Output = Color64;

    fn mul(self, rhs: Color64) -> Self::Output {
        Color64 {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
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

    pub fn sqrt(&self) -> Color64 {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
            a: self.a.sqrt(),
        }
    }

    pub fn abs(&self) -> Color64 {
        Color {
            r: self.r.abs(),
            g: self.g.abs(),
            b: self.b.abs(),
            a: self.a.abs(),
        }
    }

    pub fn horizontal_add(&self) -> f64 {
        self.r + self.g + self.b + self.a
    }

    pub fn horizontal_max(&self) -> f64 {
        self.r.max(self.g).max(self.b).max(self.a)
    }
}

impl Default for Color64 {
    fn default() -> Self {
        Self { r: 0f64, g: 0f64, b: 0f64, a: 0f64 }
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
    // FIXME: Color64 と異なって不自然なので直す
    fn default() -> Self {
        Self { r: 255, g: 255, b: 255, a: 255 }
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
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                let r = raw_buffer[4 * i];
                let g = raw_buffer[4 * i + 1];
                let b = raw_buffer[4 * i + 2];
                let a = raw_buffer[4 * i + 3];

                // as bottom is origin
                let dst_i = (height - 1 - y) * width + x;
                buffer[dst_i] = Color::new(r, g, b, a);
            }
        }
        Image { height, width, buffer }
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
    pub y: usize,
    pub x: usize,
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

enum Direction {
    Up,
    Right,
    Down,
    Left,
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
        self.left() <= pos.x && pos.x <= self.right() && self.bottom() <= pos.y && pos.y <= self.top()
    }

    /// 辺上にも存在しない
    pub fn is_internal(&self, pos: &Pos) -> bool {
        self.left() < pos.x && pos.x < self.right() && self.bottom() < pos.y && pos.y < self.top()
    }

    /// x is exclusive for left
    fn vertical_split(&self, x: usize) -> (Rectangle, Rectangle) {
        assert!(self.left() < x && x < self.right());
        let left = Rectangle::new(self.bottom(), self.left(), self.height, x - self.left());
        let right = Rectangle::new(self.bottom(), x, self.height, self.right() + 1 - x);
        (left, right)
    }

    /// y is explicit for top
    fn horizontal_split(&self, y: usize) -> (Rectangle, Rectangle) {
        assert!(self.bottom() < y && y < self.top());
        let bottom = Rectangle::new(self.bottom(), self.left(), y - self.bottom(), self.width);
        let top = Rectangle::new(y, self.left(), self.top() + 1 - y, self.width);
        (bottom, top)
    }

    fn point_split(&self, p: &Pos) -> (Rectangle, Rectangle, Rectangle, Rectangle) {
        assert!(self.is_internal(p));
        let (left, right) = self.vertical_split(p.x);
        let (bottom_left, top_left) = left.horizontal_split(p.y);
        let (bottom_right, top_right) = right.horizontal_split(p.y);

        (bottom_left, bottom_right, top_right, top_left)
    }

    // dir: self から見てどの方向に接続するか
    fn merge(&self, rect2: &Rectangle, dir: Direction) -> Rectangle {
        match dir {
            Direction::Up => Rectangle::new(self.bottom(), self.left(), self.height + rect2.height, self.width),
            Direction::Right => Rectangle::new(self.bottom(), self.left(), self.height, self.width + rect2.width),
            Direction::Down => Rectangle::new(rect2.bottom(), rect2.left(), self.height + rect2.height, self.width),
            Direction::Left => Rectangle::new(rect2.bottom(), rect2.left(), self.height, self.width + rect2.width),
        }
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
    pub index_of: usize,
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    // block_idx, y
    HorizontalSplit(usize, usize),
    // block_idx, x
    VerticalSplit(usize, usize),
    // block_idx, (x, y)
    PointSplit(usize, Pos),
    // block_idx, prev_color, color
    Color(usize, Color8),
    // block_idx, block_idx
    Swap(usize, usize),
    // block_idx, block_idx,
    Merge(usize, usize),
}

impl Command {
    pub fn base_cost(&self) -> usize {
        match *self {
            Command::HorizontalSplit(_, _) => 7,
            Command::VerticalSplit(_, _) => 7,
            Command::PointSplit(_, _) => 10,
            Command::Color(_, _) => 5,
            Command::Swap(_, _) => 3,
            Command::Merge(_, _) => 1,
        }
    }

    pub fn block_index(&self) -> usize {
        match *self {
            Command::HorizontalSplit(block_index, _) => block_index,
            Command::VerticalSplit(block_index, _) => block_index,
            Command::PointSplit(block_index, _) => block_index,
            Command::Color(block_index, _) => block_index,
            // FIXME: block_index 意味ない！！
            Command::Swap(block_index, _) => block_index,
            Command::Merge(block_index, _) => block_index,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum CommandWithLog {
    // block_idx, y
    HorizontalSplit(usize, usize),
    // block_idx, x
    VerticalSplit(usize, usize),
    // block_idx, (x, y)
    PointSplit(usize, Pos),
    // block_idx, prev_color, color
    Color(usize, Color8, Color8),
    // block_idx, block_idx
    Swap(usize, usize),
    // block_idx, block_idx
    Merge(usize, usize),
}

impl CommandWithLog {
    pub fn base_cost(&self) -> usize {
        match *self {
            CommandWithLog::HorizontalSplit(_, _) => 7,
            CommandWithLog::VerticalSplit(_, _) => 7,
            CommandWithLog::PointSplit(_, _) => 10,
            CommandWithLog::Color(_, _, _) => 5,
            CommandWithLog::Swap(_, _) => 3,
            CommandWithLog::Merge(_, _) => 1,
        }
    }

    pub fn block_index(&self) -> usize {
        match *self {
            CommandWithLog::HorizontalSplit(block_index, _) => block_index,
            CommandWithLog::VerticalSplit(block_index, _) => block_index,
            CommandWithLog::PointSplit(block_index, _) => block_index,
            CommandWithLog::Color(block_index, _, _) => block_index,
            CommandWithLog::Swap(block_index, _) => block_index,
            CommandWithLog::Merge(block_index, _) => block_index,
        }
    }

    pub fn to_command(&self) -> Command {
        match *self {
            CommandWithLog::HorizontalSplit(block_index, y) => Command::HorizontalSplit(block_index, y),
            CommandWithLog::VerticalSplit(block_index, x) => Command::VerticalSplit(block_index, x),
            CommandWithLog::PointSplit(block_index, pos) => Command::PointSplit(block_index, pos),
            CommandWithLog::Color(block_index, prev_color, color) => Command::Color(block_index, color),
            CommandWithLog::Swap(block_index1, block_index2) => Command::Swap(block_index1, block_index2),
            CommandWithLog::Merge(block_index1, block_index2) => Command::Merge(block_index1, block_index2),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub block_list: Vec<Block>,
    next_block_id: usize,
    command_list: Vec<CommandWithLog>,
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
            next_block_id: 1,
        }
    }

    pub fn get_command_list(&self) -> Vec<Command> {
        self.command_list.iter().map(|cmd| cmd.to_command()).collect()
    }

    pub fn create_with_config(config: &config_loader::TwinImageConfig) -> State {
        let mut state = State::new(config.height, config.width);
        state.block_list[0].is_child = false;

        for block_config in config.blocks.iter() {
            state.block_list.push(Block {
                rect: block_config.rect,
                color: block_config.color,
                parent: None,
                id: block_config.id,
                is_child: true,
                index_of: state.block_list.len(),
            });
            state.next_block_id = state.next_block_id.max(block_config.id);
        }
        state.next_block_id += 1;

        state
    }

    pub fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::HorizontalSplit(block_index, y) => {
                self.horizontal_split(block_index, y);
                self.command_list.push(CommandWithLog::HorizontalSplit(block_index, y));
            }
            Command::VerticalSplit(block_index, x) => {
                self.vertical_split(block_index, x);
                self.command_list.push(CommandWithLog::VerticalSplit(block_index, x));
            }
            Command::PointSplit(block_index, pos) => {
                self.point_cut(block_index, &pos);
                self.command_list.push(CommandWithLog::PointSplit(block_index, pos))
            }
            Command::Color(block_index, color) => {
                let prev_color = self.block_list[block_index].color;
                self.color(block_index, &prev_color, &color);
                self.command_list.push(CommandWithLog::Color(block_index, prev_color, color));
            }
            Command::Swap(block_index1, block_index2) => {
                self.swap(block_index1, block_index2);
                self.command_list.push(CommandWithLog::Swap(block_index1, block_index2));
            }
            Command::Merge(block_index1, block_index2) => {
                self.merge(block_index1, block_index2);
                self.command_list.push(CommandWithLog::Merge(block_index1, block_index2));
            }
        }
    }

    pub fn undo(&mut self) {
        assert!(!self.command_list.is_empty());
        match *self.command_list.last().unwrap() {
            CommandWithLog::HorizontalSplit(block_index, _) | CommandWithLog::VerticalSplit(block_index, _) => {
                for _ in 0..2 {
                    assert!(self.block_list.last().unwrap().parent.unwrap() == block_index);
                    self.block_list.pop();
                }
                assert!(!self.block_list[block_index].is_child);
                self.block_list[block_index].is_child = true;
            }
            CommandWithLog::PointSplit(block_index, _) => {
                for _ in 0..4 {
                    assert!(self.block_list.last().unwrap().parent.unwrap() == block_index);
                    self.block_list.pop();
                }
                assert!(!self.block_list[block_index].is_child);
                self.block_list[block_index].is_child = true;
            }
            CommandWithLog::Color(block_index, prev_color, _) => {
                assert!(self.block_list[block_index].is_child);
                self.block_list[block_index].color = prev_color;
            }
            CommandWithLog::Swap(_, _) => {
                unimplemented!();
            }
            CommandWithLog::Merge(_, _) => {
                unimplemented!();
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

    fn merge(&mut self, block_index1: usize, block_index2: usize) {
        let rect1 = self.block_list[block_index1].rect;
        let rect2 = self.block_list[block_index2].rect;

        let vertical_adjusted = rect1.left() == rect2.left() && rect1.right() == rect2.right();
        let horizontal_adjustted = rect1.top() == rect2.top() && rect1.bottom() == rect2.bottom();

        let rect1_bottom_connectable = rect1.bottom() == rect2.top() + 1 && vertical_adjusted;
        let rect1_top_connectable = rect1.top() + 1 == rect2.bottom() && vertical_adjusted;
        let rect1_left_connectable = rect1.left() == rect2.right() + 1 && horizontal_adjustted;
        let rect1_right_connectable = rect1.right() + 1 == rect2.left() && horizontal_adjustted;

        assert!(rect1_bottom_connectable || rect1_top_connectable || rect1_left_connectable || rect1_right_connectable);

        self.block_list[block_index1].is_child = false;
        self.block_list[block_index2].is_child = false;

        let dir = if rect1_bottom_connectable {
            Direction::Down
        } else if rect1_top_connectable {
            Direction::Up
        } else if rect1_left_connectable {
            Direction::Left
        } else {
            Direction::Right
        };

        // FIXME: merge 後のオブジェクトには必ず色を付ける制約がある(整合性が取れていない)
        let merged_block = Block {
            rect: rect1.merge(&rect2, dir),
            color: self.block_list[block_index1].color,
            parent: None, // fixme: 整合性確認
            id: self.next_block_id,
            is_child: true,
            index_of: self.block_list.len(),
        };
        self.next_block_id += 1;
        self.block_list.push(merged_block);
    }

    fn swap(&mut self, block_index1: usize, block_index2: usize) {
        assert_eq!(self.block_list[block_index1].rect.height, self.block_list[block_index2].rect.height);
        assert_eq!(self.block_list[block_index1].rect.width, self.block_list[block_index2].rect.width);
        macro_rules! local_swap {
            ($x:ident) => {{
                let tmp = self.block_list[block_index1].$x;
                self.block_list[block_index1].$x = self.block_list[block_index2].$x;
                self.block_list[block_index2].$x = tmp;
            }};
        }
        // color, id, parent を差し替え
        local_swap!(parent);
        local_swap!(rect);
        local_swap!(id);
    }

    pub fn save_image(&self, image_fliepath: &String) {
        let path = Path::new(image_fliepath);
        let file = File::create(path).unwrap();
        let ref mut writer = BufWriter::new(file);
        let data = self.to_color_buffer();
        let height = data.len();
        let width = data[0].len();
        let mut encoder = png::Encoder::new(writer, width as u32, height as u32);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
        let mut writer = encoder.write_header().unwrap();

        let mut raw_data = vec![];
        for y in (0..height).rev() {
            for x in 0..width {
                let color = data[y][x];
                let mut color_data = vec![color.r, color.g, color.b, color.a];
                raw_data.append(&mut color_data);
            }
        }
        writer.write_image_data(&raw_data).unwrap();
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
            id_list.into_iter().map(|v: usize| v.to_string()).collect::<Vec<_>>().join(".")
        };

        for cmd in self.command_list.iter() {
            match *cmd {
                CommandWithLog::HorizontalSplit(block_index, y) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("cut [{}] [y] [{}]", block_id, y);
                }
                CommandWithLog::VerticalSplit(block_index, x) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("cut [{}] [x] [{}]", block_id, x);
                }
                CommandWithLog::PointSplit(block_index, pos) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("cut [{}] [{}, {}]", block_id, pos.x, pos.y);
                }
                CommandWithLog::Color(block_index, _, color) => {
                    let block_id = restore_id_sequence(block_index);
                    println!("color [{}] [{}, {}, {}, {}] ", block_id, color.r, color.g, color.b, color.a);
                }
                CommandWithLog::Swap(block_index1, block_index2) => {
                    let block_id1 = restore_id_sequence(block_index1);
                    let block_id2 = restore_id_sequence(block_index2);
                    println!("swap [{}] [{}]", block_id1, block_id2);
                }
                CommandWithLog::Merge(block_index1, block_index2) => {
                    let block_id1 = restore_id_sequence(block_index1);
                    let block_id2 = restore_id_sequence(block_index2);
                    println!("merge [{}] [{}]", block_id1, block_id2);
                }
            }
        }
    }

    pub fn to_color_buffer(&self) -> Vec<Vec<Color8>> {
        let width = self.block_list[0].rect.width;
        let height = self.block_list[0].rect.height;

        let mut ret = vec![vec![Color8::default(); width]; height];
        // FIXME: 色の変化があったブロックを追従する
        for block in self.block_list.iter() {
            let rect = block.rect;
            for y in rect.bottom()..=rect.top() {
                for x in rect.left()..=rect.right() {
                    ret[y][x] = block.color;
                }
            }
        }

        ret
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

        clone.apply(Command::Color(1, Color::new(128, 128, 128, 128)));
        assert_eq!(clone.block_list.len(), 5);
        clone.undo();
        assert_eq!(state, clone);
    }
}

pub fn evaluate(image: &Image, state: &State) -> f64 {
    let mut pixel_cost = 0f64;
    let state_image = state.to_color_buffer();
    for y in 0..image.height {
        for x in 0..image.width {
            let diff = state_image[y][x].to64() - image.color_of(y, x).to64();
            pixel_cost += diff.square().horizontal_add().sqrt()
        }
    }

    let mut command_cost = 0;
    for cmd in state.command_list.iter() {
        let base_cost = cmd.base_cost();
        let block_index = cmd.block_index();
        command_cost += image.size() / state.block_list[block_index].rect.size() * base_cost;
    }
    pixel_cost = (pixel_cost * ALPHA).round();

    eprintln!("cost: (pixel, command) = ({}, {})", pixel_cost, command_cost);

    pixel_cost + command_cost as f64
}

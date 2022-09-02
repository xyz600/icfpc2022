#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new() -> Color {
        Color {
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
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommandLog {}

#[derive(Clone)]
pub struct State {
    pub block_list: Vec<Block>,
    pub command_list: Vec<CommandLog>,
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
            color: Color::new(),
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

    pub fn point_cut(&mut self, block_id: usize, pos: &Pos) {
        assert!(block_id < self.block_list.len());
        let mut parent_block = &mut self.block_list[block_id];
        assert!(parent_block.id == block_id);
        assert!(parent_block.rect.is_internal(pos));
        assert!(parent_block.is_child);
        parent_block.is_child = false;
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
}

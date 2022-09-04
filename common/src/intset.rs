use crate::random::CachedRandom;

const INVALID: usize = std::usize::MAX;

pub struct IntSet {
    buffer: Vec<usize>,
    index_of: Vec<usize>,
    size: usize,
}

impl IntSet {
    pub fn new(n: usize) -> IntSet {
        IntSet {
            buffer: vec![0; n + 1],
            index_of: vec![INVALID; n + 1],
            size: 0,
        }
    }

    pub fn add(&mut self, v: usize) {
        self.buffer[self.size] = v;
        self.index_of[v] = self.size;
        self.size += 1;
    }

    pub fn contains(&self, v: usize) -> bool {
        self.index_of[v] != INVALID
    }

    pub fn remove(&mut self, v: usize) {
        let target_index = self.index_of[v];
        let move_index = self.size - 1;
        let move_val = self.buffer[move_index];

        self.buffer[target_index] = move_val;
        self.index_of[v] = INVALID;
        self.index_of[move_val] = target_index;
        self.size -= 1;
    }

    pub fn choose(&self, rand: &mut CachedRandom) -> usize {
        let index = rand.next_int_range(0, self.size as u32 - 1);
        self.buffer[index as usize]
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

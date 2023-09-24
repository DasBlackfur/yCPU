use std::ops::{Index, IndexMut, Range};

pub struct Banker<T> {
    pub content: [T; 256],
    pub pointer: u8,
}

impl<T: std::marker::Copy + Index<usize>> Banker<T> {
    pub fn new(content: T) -> Banker<T> {
        Banker {
            content: [content; 256],
            pointer: 0,
        }
    }
}

impl<T: Index<usize>> Index<usize> for Banker<T> {
    type Output = <T as Index<usize>>::Output;
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[self.pointer as usize][index]
    }
}

impl<T: IndexMut<usize>> IndexMut<usize> for Banker<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[self.pointer as usize][index]
    }
}

impl<T: Index<Range<usize>>> Index<Range<usize>> for Banker<T> {
    type Output = <T as Index<Range<usize>>>::Output;
    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.content[self.pointer as usize][index]
    }
}

impl<T: IndexMut<Range<usize>>> IndexMut<Range<usize>> for Banker<T> {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        &mut self.content[self.pointer as usize][index]
    }
}

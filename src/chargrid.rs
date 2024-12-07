use core::str;
use std::{
    collections::HashSet,
    fmt,
    ops::{Add, Index, IndexMut, Mul, Sub},
    slice::{Chunks, ChunksMut},
};

extern crate test;

pub struct ByteGrid {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, Default)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Point {
        Point { row, col }
    }

    fn adjacencies(self) -> [Point; 8] {
        [
            self + NORTHWEST,
            self + NORTH,
            self + NORTHEAST,
            self + WEST,
            self + EAST,
            self + SOUTHWEST,
            self + SOUTH,
            self + SOUTHEAST,
        ]
    }

    fn orthogonals(self) -> [Point; 4] {
        [self + NORTH, self + WEST, self + EAST, self + SOUTH]
    }

    fn diagonals(self) -> [Point; 4] {
        [
            self + NORTHWEST,
            self + NORTHEAST,
            self + SOUTHWEST,
            self + SOUTHEAST,
        ]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            row: self.row * self.row,
            col: self.col * self.col,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            row: self.row * other,
            col: self.col * other,
        }
    }
}

pub const NORTHWEST: Point = Point { row: -1, col: -1 };
pub const NORTH: Point = Point { row: -1, col: 0 };
pub const NORTHEAST: Point = Point { row: -1, col: 1 };
pub const WEST: Point = Point { row: 0, col: -1 };
pub const ORIGIN: Point = Point { row: 0, col: 0 };
pub const EAST: Point = Point { row: 0, col: 1 };
pub const SOUTHWEST: Point = Point { row: 1, col: -1 };
pub const SOUTH: Point = Point { row: 1, col: 0 };
pub const SOUTHEAST: Point = Point { row: 1, col: 1 };

pub struct ByteGridIter<'a> {
    grid: &'a ByteGrid,
    pub row: i32,
    pub col: i32,
    row_offset: i32,
    col_offset: i32,
}

impl<'a> ByteGridIter<'a> {
    pub fn new(grid: &'a ByteGrid, row: i32, col: i32, row_offset: i32, col_offset: i32) -> Self {
        Self {
            grid,
            row: row - row_offset,
            col: col - col_offset,
            row_offset,
            col_offset,
        }
    }
}

impl<'a> Iterator for ByteGridIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.row += self.row_offset;
        if self.row < 0 || self.row >= self.grid.height {
            return None;
        }
        self.col += self.col_offset;
        if self.col < 0 || self.col >= self.grid.width {
            return None;
        }
        return Some(self.grid.data[(self.row * self.grid.width + self.col) as usize]);
    }
}

pub struct ByteGridIterMut<'a> {
    grid: &'a mut ByteGrid,
    pub row: i32,
    pub col: i32,
    row_offset: i32,
    col_offset: i32,
}

impl<'a> ByteGridIterMut<'a> {
    pub fn new(
        grid: &'a mut ByteGrid,
        row: i32,
        col: i32,
        row_offset: i32,
        col_offset: i32,
    ) -> Self {
        Self {
            grid,
            row: row - row_offset,
            col: col - col_offset,
            row_offset,
            col_offset,
        }
    }
}

impl<'a> Iterator for ByteGridIterMut<'a> {
    type Item = &'a mut u8;

    fn next(&'_ mut self) -> Option<Self::Item> {
        self.row += self.row_offset;
        if self.row < 0 || self.row >= self.grid.height {
            return None;
        }
        self.col += self.col_offset;
        if self.col < 0 || self.col >= self.grid.width {
            return None;
        }
        let idx = (self.row * self.grid.width + self.col) as usize;
        if idx >= self.grid.data.len() {
            return None;
        }
        unsafe {
            let ptr = self.grid.data.as_mut_ptr();
            return Some(&mut *ptr.add(idx));
        }
    }
}

impl ByteGrid {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let data: Vec<u8> = input
            .as_bytes()
            .iter()
            .filter_map(|c| if *c != b'\n' { Some(*c) } else { None })
            .collect();
        let height = data.len() / width;
        Self {
            width: width as i32,
            height: height as i32,
            data: data,
        }
    }

    #[inline]
    pub fn coord_to_idx(&self, row: i32, col: i32) -> usize {
        (row * self.width + col) as usize
    }

    #[inline]
    pub fn idx_to_coord(&self, idx: usize) -> (i32, i32) {
        let idx_u32 = idx as i32;
        (idx_u32 / self.width, idx_u32 - (idx_u32 / self.width))
    }

    #[inline]
    pub fn idx_to_point(&self, idx: usize) -> Point {
        let idx_u32 = idx as i32;
        Point {
            row: idx_u32 / self.width,
            col: idx_u32 - (idx_u32 / self.width),
        }
    }

    pub fn rows(&self) -> Chunks<u8> {
        self.data.chunks(self.width as usize)
    }

    pub fn rows_mut(&mut self) -> ChunksMut<u8> {
        self.data.chunks_mut(self.width as usize)
    }

    pub fn get_point(&self, p: Point) -> Option<&u8> {
        self.data.get(self.coord_to_idx(p.row, p.col))
    }

    pub fn get_point_mut(&mut self, p: Point) -> Option<&mut u8> {
        self.data.get_mut((p.row * self.width + p.col) as usize)
    }

    #[inline]
    pub fn is_valid_point(&self, p: Point) -> bool {
        0 <= p.row && p.row < self.height && 0 <= p.col && p.col < self.width
    }

    pub fn flood_adjacencies(
        &mut self,
        start: Point,
        mut f: impl FnMut(&mut u8, Point) -> (),
    ) -> usize {
        let start_u8 = match self.get_point_mut(start) {
            Some(c) => c.clone(),
            None => return 0,
        };
        let mut visited: HashSet<Point> = HashSet::new();
        let mut stack = vec![start];
        let mut total = 0;
        while let Some(point) = stack.pop() {
            visited.insert(point);
            let c = &mut self[point];
            if *c == start_u8 {
                f(c, point);
                total += 1;
                for adj in point.adjacencies() {
                    if self.is_valid_point(adj) && !visited.contains(&adj) {
                        stack.push(adj);
                    }
                }
            }
        }
        total
    }

    pub fn flood_orthogonals(
        &mut self,
        start: Point,
        mut f: impl FnMut(&mut u8, Point) -> (),
    ) -> usize {
        let start_u8 = match self.get_point_mut(start) {
            Some(c) => c.clone(),
            None => return 0,
        };
        let mut visited: HashSet<Point> = HashSet::new();
        let mut stack = vec![start];
        let mut total = 0;
        while let Some(point) = stack.pop() {
            visited.insert(point);
            let c = &mut self[point];
            if *c == start_u8 {
                f(c, point);
                total += 1;
                for adj in point.orthogonals() {
                    if self.is_valid_point(adj) && !visited.contains(&adj) {
                        stack.push(adj);
                    }
                }
            }
        }
        total
    }

    pub fn u8s_surrounding(&self, point: Point, offsets: &[Point]) -> Option<Vec<u8>> {
        let mut result: Vec<u8> = Vec::with_capacity(offsets.len());
        for offset in offsets.iter() {
            let new_point = point + *offset;
            if let Some(c) = self.get_point(new_point) {
                result.push(*c);
            } else {
                return None;
            }
        }
        return Some(result);
    }

    // Directional iterators
    pub fn iter_from_point_with_offsets(
        &self,
        row: i32,
        col: i32,
        row_offset: i32,
        col_offset: i32,
    ) -> ByteGridIter {
        ByteGridIter::new(self, row, col, row_offset, col_offset)
    }

    pub fn iter_mut_ray(
        &mut self,
        row: i32,
        col: i32,
        row_offset: i32,
        col_offset: i32,
    ) -> ByteGridIterMut {
        ByteGridIterMut::new(self, row, col, row_offset, col_offset)
    }

    pub fn iter_towards(&self, pos: Point, vel: Point) -> ByteGridIter {
        self.iter_from_point_with_offsets(pos.row, pos.col, vel.row, vel.col)
    }

    pub fn iter_mut_towards(&mut self, pos: Point, vel: Point) -> ByteGridIterMut {
        self.iter_mut_ray(pos.row, pos.col, vel.row, vel.col)
    }
}

impl fmt::Display for ByteGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            writeln!(f, "{}", str::from_utf8(row).unwrap())?;
        }
        fmt::Result::Ok(())
    }
}

impl Index<Point> for ByteGrid {
    type Output = u8;

    fn index(&self, point: Point) -> &Self::Output {
        self.data
            .index((point.row * self.width + point.col) as usize)
    }
}

impl IndexMut<Point> for ByteGrid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        self.data
            .index_mut((point.row * self.width + point.col) as usize)
    }
}

impl Index<(i32, i32)> for ByteGrid {
    type Output = u8;

    fn index(&self, point: (i32, i32)) -> &Self::Output {
        self.data.index((point.0 * self.width + point.1) as usize)
    }
}

impl IndexMut<(i32, i32)> for ByteGrid {
    fn index_mut(&mut self, point: (i32, i32)) -> &mut Self::Output {
        self.data
            .index_mut((point.0 * self.width + point.1) as usize)
    }
}

impl Index<usize> for ByteGrid {
    type Output = u8;

    fn index(&self, idx: usize) -> &Self::Output {
        self.data.index(idx)
    }
}

impl IndexMut<usize> for ByteGrid {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.data.index_mut(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_point() {
        let origin = Point::default();
        assert_eq!(origin.row, 0);
        assert_eq!(origin.col, 0);
        let new_point = origin + SOUTHEAST * 2 - SOUTH;
        assert_eq!(new_point.row, 1);
        assert_eq!(new_point.col, 2);
    }

    #[test]
    fn test_u8_grid() {
        let mut grid = ByteGrid::new(
            "0123456789\n\
             11......1.\n\
             2.2....2..\n\
             3..3..3...\n\
             4...44....\n\
             5...55....\n\
             6..6..6...\n\
             7.7....7..\n\
             88......8.\n\
             9........9\n",
        );
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.data[99], b'9');

        let column_vec: Vec<u8> = grid.iter_towards(ORIGIN + SOUTH, SOUTH).collect();
        assert_eq!(
            column_vec,
            vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']
        );

        let row_vec: Vec<u8> = grid.iter_towards(ORIGIN + SOUTH, EAST).collect();
        assert_eq!(
            row_vec,
            vec![b'1', b'1', b'.', b'.', b'.', b'.', b'.', b'.', b'1', b'.']
        );

        for x in grid.iter_mut_ray(0, 0, 1, 0) {
            *x = b'!';
        }

        let column_vec: Vec<u8> = grid.iter_towards(ORIGIN + SOUTH, SOUTH).collect();
        assert_eq!(
            column_vec,
            vec![b'!', b'!', b'!', b'!', b'!', b'!', b'!', b'!', b'!']
        );
        assert_eq!(grid.get_point(ORIGIN), Some(&b'!'));
        assert_eq!(grid[ORIGIN], b'!');
        assert_eq!(grid[(0, 0)], b'!');
        assert_eq!(grid[0], b'!');

        let mut blah = 0;
        let count = grid.flood_orthogonals(Point::new(2, 3), |c, p| {
            if p.col == 4 {
                blah += 1;
            }
            *c = b'%'
        });

        assert_eq!(count, 12);
        assert_eq!(blah, 3);
        assert_eq!(grid[(2, 3)], b'%');
        assert_eq!(grid[(1, 3)], b'%');
        assert_eq!(grid[(1, 1)], b'1');

        let s = format!("{}", grid);
        assert_eq!(
            s,
            "!123456789\n\
             !1%%%%%%1.\n\
             !.2%%%%2..\n\
             !..3%%3...\n\
             !...44....\n\
             !...55....\n\
             !..6..6...\n\
             !.7....7..\n\
             !8......8.\n\
             !........9\n",
        )
    }

    const FLOOD_TEST: &str = "#################################################\n\
                              1.1............................................1.\n\
                              2..2..........................................2..\n\
                              3...3........................................3...\n\
                              4..5........................................4....\n\
                              5.5.........................................5....\n\
                              6.6..........................................6...\n\
                              7.7...........................................7..\n\
                              8.8............................................8.\n\
                              8.8............................................8.\n\
                              8.8............................................8.\n\
                              8.8............................................8.\n\
                              8.8............................................8.\n\
                              ######.##########################################\n\
                              8...##.#.........................#.............8.\n\
                              8.8....#.........................#.............8.\n\
                              8.8#####.........................#.............8.\n\
                              8.8..............................#.............8.\n\
                              8.8..............................#.............8.\n\
                              8.8..............................#.............8.\n\
                              #################################################\n";

    #[bench]
    fn bench_flood(b: &mut Bencher) {
        let mut grid = ByteGrid::new(FLOOD_TEST);
        b.iter(|| {
            let mut blah = 0;
            grid.flood_orthogonals(Point::new(5, 5), |c, p| {
                if p.col == 4 {
                    blah += 1;
                }
                *c = b' '
            });
        });
    }
}

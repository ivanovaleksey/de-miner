extern crate itertools;

#[derive(Debug, PartialEq)]
enum CellKind {
    Bomb,
    Safe,
}

#[derive(Debug, PartialEq)]
enum OpenCellKind {
    Bomb,
    Safe(usize),
}

#[derive(Debug)]
struct Field {
    size: usize,
    cells: Vec<CellKind>,
}

impl Field {
    fn parse(input: &str) -> Field {
        let size = input.lines().count();
        let mut cells = Vec::with_capacity(size * size);

        for line in input.lines() {
            for c in line.trim().split(' ') {
                if c.is_empty() {
                    break;
                }

                let cell = match c {
                    "X" => CellKind::Bomb,
                    "O" => CellKind::Safe,
                    _ => unreachable!(),
                };

                cells.push(cell);
            }
        }

        Field { size, cells }
    }

    fn open(&self) -> Vec<OpenCellKind> {
        let mut cells = Vec::with_capacity(self.size * self.size);

        for (idx, cell) in self.cells.iter().enumerate() {
            let open_cell = match *cell {
                CellKind::Bomb => OpenCellKind::Bomb,
                CellKind::Safe => {
                    let x = idx % self.size;
                    let y = idx / self.size;
                    let adjacent_cells = Cell(x, y).adjacent();
                    let adjacent_cells = adjacent_cells
                        .iter()
                        .filter(|cell| cell.0 < self.size && cell.1 < self.size);

                    let count = adjacent_cells
                        .filter(|cell| {
                            let index = cell.1 * self.size + cell.0;
                            match self.cells.get(index) {
                                Some(c) => match *c {
                                    CellKind::Bomb => true,
                                    CellKind::Safe => false,
                                },
                                None => false,
                            }
                        })
                        .count();

                    OpenCellKind::Safe(count)
                }
            };

            cells.push(open_cell);
        }

        cells
    }
}

impl PartialEq<Vec<CellKind>> for Field {
    fn eq(&self, other: &Vec<CellKind>) -> bool {
        &self.cells == other
    }
}

#[derive(Debug, PartialEq)]
struct Cell(usize, usize);

impl Cell {
    fn adjacent(&self) -> Vec<Cell> {
        use std::ops::Range;
        use itertools::Itertools;

        let x = Range {
            start: self.0.checked_sub(1).unwrap_or(0),
            end: self.0 + 2,
        };
        let y = Range {
            start: self.1.checked_sub(1).unwrap_or(0),
            end: self.1 + 2,
        };

        let mut cells = y.cartesian_product(x)
            .map(|pair| Cell(pair.1, pair.0))
            .collect::<Vec<_>>();

        cells.retain(|cell| cell != self);
        cells
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        use CellKind::*;

        let input = r#"X O O
                       O O O
                       X X O"#;

        let field = Field::parse(input);
        let expected = vec![Bomb, Safe, Safe, Safe, Safe, Safe, Bomb, Bomb, Safe];
        assert_eq!(field, expected);
    }

    #[test]
    fn find_adjacent_cells() {
        let cell = Cell(0, 0);
        let expected = vec![Cell(1, 0), Cell(0, 1), Cell(1, 1)];
        assert_eq!(cell.adjacent(), expected);

        let cell = Cell(1, 0);
        let expected = vec![Cell(0, 0), Cell(2, 0), Cell(0, 1), Cell(1, 1), Cell(2, 1)];
        assert_eq!(cell.adjacent(), expected);

        let cell = Cell(1, 1);
        let expected = vec![
            Cell(0, 0),
            Cell(1, 0),
            Cell(2, 0),
            Cell(0, 1),
            Cell(2, 1),
            Cell(0, 2),
            Cell(1, 2),
            Cell(2, 2),
        ];
        assert_eq!(cell.adjacent(), expected);
    }

    #[test]
    fn open_field() {
        use OpenCellKind::*;

        let input = r#"X O O
                       O O O
                       X X O"#;

        let field = Field::parse(input);
        let expected = vec![
            Bomb,
            Safe(1),
            Safe(0),
            Safe(3),
            Safe(3),
            Safe(1),
            Bomb,
            Bomb,
            Safe(1),
        ];
        assert_eq!(field.open(), expected);
    }

    // #[test]
    // fn main() {
    //     let input = r#"
    //         X O O X X X O O
    //         O O O O X O X X
    //         X X O X X O O O
    //         O X O O O X X X
    //         O O X X X X O X
    //         X O X X X O X O
    //         O O O X O X O X
    //         X O X X O X O X
    //     "#;
    // }
}

extern crate itertools;

#[derive(Debug, PartialEq)]
enum CellKind {
    Bomb,
    Safe,
}

#[derive(Debug)]
struct Field(Vec<CellKind>);

impl Field {
    fn parse(input: &str) -> Field {
        let n = input.lines().count();
        let mut field = Vec::with_capacity(n * n);

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

                field.push(cell);
            }
        }

        Field(field)
    }
}

impl PartialEq<Vec<CellKind>> for Field {
    fn eq(&self, other: &Vec<CellKind>) -> bool {
        &self.0 == other
    }
}

#[derive(Debug, PartialEq)]
struct Cell(usize, usize);

impl Cell {
    fn adjacent(&self) -> Vec<Cell> {
        use std::ops::Range;
        use itertools::Itertools;

        let cols = Range {
            start: self.0.checked_sub(1).unwrap_or(0),
            end: self.0 + 2,
        };
        let rows = Range {
            start: self.1.checked_sub(1).unwrap_or(0),
            end: self.1 + 2,
        };

        let mut cells = cols.cartesian_product(rows)
            .map(|pair| Cell(pair.0, pair.1))
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
    use CellKind::*;

    #[test]
    fn parse_input() {
        let input = r#"
            X O O
            O O O
            X X O
        "#;

        let field = Field::parse(input);
        let expected = vec![Bomb, Safe, Safe, Safe, Safe, Safe, Bomb, Bomb, Safe];
        assert_eq!(field, expected);
    }

    #[test]
    fn find_adjacent_cells() {
        let cell = Cell(0, 0);
        let expected = vec![Cell(0, 1), Cell(1, 0), Cell(1, 1)];
        assert_eq!(cell.adjacent(), expected);

        let cell = Cell(0, 1);
        let expected = vec![Cell(0, 0), Cell(0, 2), Cell(1, 0), Cell(1, 1), Cell(1, 2)];
        assert_eq!(cell.adjacent(), expected);

        let cell = Cell(1, 1);
        let expected = vec![
            Cell(0, 0),
            Cell(0, 1),
            Cell(0, 2),
            Cell(1, 0),
            Cell(1, 2),
            Cell(2, 0),
            Cell(2, 1),
            Cell(2, 2),
        ];
        assert_eq!(cell.adjacent(), expected);
    }
}

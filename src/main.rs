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
}

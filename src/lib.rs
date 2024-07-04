// What to do
// 1. Parse any strings entered to numbers // (+6, 5)
// 2. Parse any operation // (5, +)
// 3. Make Executions

mod utils;

// 1. Numbers
#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);

        (s, Self(number.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);

        let op = match op {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => unreachable!(),
        };

        (s, op)
    }
}

// 3. Expression
#[derive(Debug, PartialEq)]
pub struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, rhs) = Number::new(s);
        let (s, op) = Op::new(s);

        (s, Self { lhs, rhs, op })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        assert_eq!(
            Expr::new("1+2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                },
            ),
        );
    }

    #[test]
    fn test_extract_number() {
        assert_eq!(Number::new("6"), ("", Number(6)));
    }

    #[test]
    fn test_extract_op() {
        assert_eq!(Op::new("+6"), ("6", Op::Add));
    }
}

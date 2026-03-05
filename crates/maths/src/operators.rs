use std::fmt;
use std::str::FromStr;

// Operators for calculations
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{}", symbol)
    }
}

impl Operator {
    pub fn apply(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Operator::Add => Ok(a + b),
            Operator::Subtract => Ok(a - b),
            Operator::Multiply => Ok(a * b),
            Operator::Divide => {
                if b == 0.0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}
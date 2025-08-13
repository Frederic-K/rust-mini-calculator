use std::io::{self, Write};

// Simple arithmetic functions. We keep them small and pure to highlight types & functions.
fn add(a: f64, b: f64) -> f64 { a + b }
fn sub(a: f64, b: f64) -> f64 { a - b }
fn mul(a: f64, b: f64) -> f64 { a * b }
fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}
fn pow(a: f64, b: f64) -> f64 { a.powf(b) }

// Parse a line like: "+ 2 3" or "add 2 3"
fn evaluate(line: &str) -> Result<String, String> {
    // Trim and split by whitespace
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err(String::from("Please enter an operation, e.g. '+ 2 3'"));
    }
    if parts.len() != 3 {
        return Err(String::from("Expected 3 tokens: <op> <a> <b>"));
    }

    let op = parts[0];
    let a: f64 = parts[1].parse().map_err(|_| format!("'{}' is not a number", parts[1]))?;
    let b: f64 = parts[2].parse().map_err(|_| format!("'{}' is not a number", parts[2]))?;

    // Support both symbols and keywords to mirror "ergonomic APIs" you might write in Express.
    let result = match op {
        "+" | "add" => Ok(add(a, b)),
        "-" | "sub" => Ok(sub(a, b)),
        "*" | "mul" | "x" => Ok(mul(a, b)),
        "/" | "div" => div(a, b),
        "^" | "pow" => Ok(pow(a, b)),
        _ => Err(format!("Unknown operation '{}'. Try one of: +, -, *, /, ^ (or add, sub, mul, div, pow)", op)),
    }?;

    Ok(format!("{}", result))
}

fn print_banner() {
    println!("===============================================");
    println!(" Rust Mini Calculator (CLI)");
    println!(" - Enter: <op> <a> <b>  e.g. '+ 2 3' or 'mul 3 4'");
    println!(" - Supported ops: +, -, *, /, ^  (add, sub, mul, div, pow)");
    println!(" - Type 'exit' to quit");
    println!("===============================================");
}

fn main() {
    print_banner();

    let stdin = io::stdin();
    loop {
        print!("> ");
        // Always flush stdout after a prompt
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        // Read a line from stdin
        let bytes = stdin.read_line(&mut line).expect("failed to read line");

        // bytes == 0 means EOF (Ctrl+D / Ctrl+Z), exit gracefully
        if bytes == 0 {
            println!("\nGoodbye!");
            break;
        }

        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        match evaluate(trimmed) {
            Ok(out) => println!("{}", out),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

// --- Tests ---
// Run with: cargo test
#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn test_add() {
        assert_eq!(evaluate("+ 2 3").unwrap(), "5");
        assert_eq!(evaluate("add 2 3").unwrap(), "5");
    }

    #[test]
    fn test_div_by_zero() {
        assert!(evaluate("/ 4 0").is_err());
    }

    #[test]
    fn test_pow() {
        assert_eq!(evaluate("pow 2 3").unwrap(), "8");
    }

    #[test]
    fn test_bad_tokens() {
        assert!(evaluate("+ 2").is_err());
        assert!(evaluate("hello").is_err());
    }
}
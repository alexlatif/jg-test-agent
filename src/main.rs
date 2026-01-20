use clap::Parser;

#[derive(Parser)]
#[command(version, about = "A simple greeting CLI")]
struct Cli {
    /// Name to greet
    #[arg(long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, world!"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_greeting_default() {
        // This test validates the greeting logic, not CLI parsing
        let name: Option<String> = None;
        let greeting = match name {
            Some(n) => format!("Hello, {}!", n),
            None => "Hello, world!".to_string(),
        };
        assert_eq!(greeting, "Hello, world!");
    }

    #[test]
    fn test_greeting_with_name() {
        // This test validates the greeting logic with a name
        let name: Option<String> = Some("Alice".to_string());
        let greeting = match name {
            Some(n) => format!("Hello, {}!", n),
            None => "Hello, world!".to_string(),
        };
        assert_eq!(greeting, "Hello, Alice!");
    }
}

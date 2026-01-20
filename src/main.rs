use clap::Parser;

#[derive(Parser)]
#[command(name = "jgtest")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A test CLI application", long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_version_matches_cargo_toml() {
        let version = env!("CARGO_PKG_VERSION");
        assert_eq!(version, "0.1.0");
    }
}

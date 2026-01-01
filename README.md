# asn-parser-rs

A CLI tool to get ASN (Autonomous System Number) from IP addresses - built with Rust.

## About

This project is designed as a learning tool for both Rust programming and networking concepts. It will eventually allow you to query an IP address and retrieve its associated Autonomous System Number (ASN), which is useful for network administration, security analysis, and understanding internet routing.

## Current Status

🚀 **Starter Template** - Currently prints a hello-world message. Full ASN lookup functionality coming soon!

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.70 or later): Install from [rustup.rs](https://rustup.rs/)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Quick Start

### Building the Project

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/manugupt1/asn-parser-rs.git
cd asn-parser-rs

# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release
```

### Running the CLI

```bash
# Run in development mode
cargo run

# Run the optimized binary
./target/release/asn-parser
```

## Development Best Practices

### 1. Code Formatting

Always format your code before committing:

```bash
# Format all Rust code
cargo fmt

# Check if code is formatted without making changes
cargo fmt -- --check
```

### 2. Linting

Use Clippy (Rust's linter) to catch common mistakes and improve code quality:

```bash
# Run Clippy
cargo clippy

# Run Clippy with all warnings as errors
cargo clippy -- -D warnings
```

### 3. Testing

Write tests for your code and run them regularly:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### 4. Documentation

Document your code with doc comments:

```rust
/// This function does something important
///
/// # Arguments
///
/// * `arg1` - Description of arg1
///
/// # Examples
///
/// ```
/// let result = my_function(42);
/// ```
fn my_function(arg1: i32) -> i32 {
    arg1 * 2
}
```

Generate and view documentation:

```bash
# Generate documentation
cargo doc

# Generate and open documentation in browser
cargo doc --open
```

### 5. Dependency Management

Keep dependencies up to date and minimal:

```bash
# Update dependencies
cargo update

# Check for outdated dependencies (requires cargo-outdated)
cargo outdated

# Audit dependencies for security vulnerabilities (requires cargo-audit)
cargo audit
```

### 6. Error Handling

- Use `Result<T, E>` for functions that can fail
- Use the `?` operator for propagating errors
- Create custom error types for better error messages
- Avoid `unwrap()` and `expect()` in production code

### 7. Project Structure

As the project grows, organize code into modules:

```
asn-parser-rs/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library code (if needed)
│   ├── cli.rs          # CLI argument parsing
│   ├── asn/            # ASN lookup logic
│   │   ├── mod.rs
│   │   └── lookup.rs
│   └── network/        # Network utilities
│       ├── mod.rs
│       └── ip.rs
└── tests/              # Integration tests
    └── integration_test.rs
```

## Learning Resources

### Rust Programming

- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust programming guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive Rust exercises
- [Rust Standard Library](https://doc.rust-lang.org/std/) - Standard library documentation

### Networking Concepts

- **ASN (Autonomous System Number)**: A unique identifier for a network on the internet
- **BGP (Border Gateway Protocol)**: The protocol that uses ASNs for routing
- **WHOIS**: Protocol for querying information about IP addresses and domains
- **IP Address Geolocation**: Determining the geographic location of an IP

### Recommended Crates for This Project

- [`clap`](https://docs.rs/clap/) - Command-line argument parsing
- [`reqwest`](https://docs.rs/reqwest/) - HTTP client for making API requests
- [`serde`](https://docs.rs/serde/) and [`serde_json`](https://docs.rs/serde_json/) - Serialization/deserialization
- [`tokio`](https://docs.rs/tokio/) - Asynchronous runtime
- [`ipnetwork`](https://docs.rs/ipnetwork/) - IP address and network manipulation

## Roadmap

- [x] Create starter CLI template
- [ ] Add command-line argument parsing (IP address input)
- [ ] Implement ASN lookup using public APIs (e.g., ipapi.co, team-cymru) or local databases (e.g., MaxMind GeoIP2, IP2Location)
- [ ] Add error handling and validation
- [ ] Create comprehensive tests
- [ ] Add configuration file support
- [ ] Publish to crates.io

## Contributing

Contributions are welcome! This is a learning project, so feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting (`cargo test && cargo clippy`)
5. Format your code (`cargo fmt`)
6. Commit your changes (`git commit -m 'Add some amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Rust community for excellent documentation and tools
- Inspired by the need to learn networking concepts through practical coding

---

**Happy Learning!** 🦀🌐 

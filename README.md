# asn-fetcher-rs

A CLI tool to fetch ASN (Autonomous System Number) information from IP addresses - built with Rust.

## About

This project is designed as a learning tool for both Rust programming and networking concepts. It will eventually allow you to query an IP address and retrieve its associated Autonomous System Number (ASN), which is useful for network administration, security analysis, and understanding internet routing.

## Current Status

✅ **Fully Functional** - ASN lookup using RIPE NCC API is now implemented!

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
git clone https://github.com/manugupt1/asn-fetcher-rs.git
cd asn-fetcher-rs

# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release
```

### Running the CLI

```bash
# Run in development mode with an IP address
cargo run -- --ip 8.8.8.8

# Run the optimized binary
./target/release/asn-fetcher --ip 8.8.8.8

# IPv6 support
cargo run -- --ip 2001:4860:4860::8888
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
asn-fetcher-rs/
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

## Release Process

This project uses an automated release script to manage versions and publish to crates.io.

### Creating a Release

To create a new release, use the release script with a semantic version bump type:

```bash
# Patch release (0.1.0 -> 0.1.1) - for bug fixes
./scripts/release.sh patch

# Minor release (0.1.0 -> 0.2.0) - for new features
./scripts/release.sh minor

# Major release (0.1.0 -> 1.0.0) - for breaking changes
./scripts/release.sh major
```

### What the Script Does

1. **Verifies Git State**: Ensures your working directory is clean and you're ready to release
2. **Runs Quality Checks**: Executes tests, clippy, and format checks
3. **Bumps Version**: Updates version in `Cargo.toml` based on the bump type
4. **Creates Git Commit**: Commits the version change with message `chore: bump version to vX.Y.Z`
5. **Creates Git Tag**: Tags the commit with `vX.Y.Z`
6. **Pushes to GitHub**: Pushes both the commit and tag to the remote repository
7. **Triggers CI/CD**: GitHub Actions automatically publishes to crates.io and creates a GitHub Release

### Release Workflow

The automated publish workflow (`.github/workflows/publish.yml`) is triggered when you push a tag matching `v*.*.*`. It will:

- Run all quality checks (tests, clippy, format validation)
- Verify the tag version matches `Cargo.toml` version
- Publish the crate to crates.io (using `CARGO_REGISTRY_TOKEN` secret)
- Create a GitHub Release with auto-generated release notes

### Prerequisites for Publishing

Before your first release, ensure:

1. You have collaborator access to the GitHub repository
2. The repository has a `CARGO_REGISTRY_TOKEN` secret configured (for crates.io publishing)
3. All tests pass and code is properly formatted

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
- [x] Add command-line argument parsing (IP address input)
- [x] Implement ASN lookup using RIPE NCC API
- [x] Add error handling and validation
- [x] Create comprehensive tests
- [ ] Add configuration file support
- [ ] Support additional ASN data sources (ipapi.co, team-cymru, MaxMind GeoIP2)
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

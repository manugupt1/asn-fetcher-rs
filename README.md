# asn-fetcher-rs

A CLI tool to fetch ASN (Autonomous System Number) information from IP addresses - built with Rust.

## About

This tool queries an IP address and retrieves its associated Autonomous System Number (ASN), useful for network administration, security analysis, and understanding internet routing.

## Features

- **Multiple ASN Data Sources**: RIPE NCC API and ipapi.co support
- **IPv4 and IPv6 Support**: Query any IP address type
- **Fast and Reliable**: Built with Rust for performance and safety
- **Simple CLI**: Easy-to-use command-line interface

## Current Status

✅ **Fully Functional** - ASN lookup with RIPE NCC API and ipapi.co

## Prerequisites

Before you begin, ensure you have Rust installed (1.70 or later):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/manugupt1/asn-fetcher-rs.git
cd asn-fetcher-rs

# Build the project
cargo build --release
```

## Usage

### Basic Usage

```bash
# Using RIPE NCC API (default)
cargo run -- 8.8.8.8

# Using ipapi.co
cargo run -- 8.8.8.8 --source ipapi

# IPv6 support
cargo run -- 2001:4860:4860::8888

# Run the optimized binary directly
./target/release/asn-fetcher 8.8.8.8
```

### API Keys

For ipapi.co with higher rate limits, set the `IPAPI_API_KEY` environment variable:

```bash
export IPAPI_API_KEY=your_api_key_here
cargo run -- 8.8.8.8 --source ipapi
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check formatting
cargo fmt -- --check
```

### Creating a Release

Use the release script to create a new version:

```bash
./scripts/release.sh patch  # 0.1.0 -> 0.1.1
./scripts/release.sh minor  # 0.1.0 -> 0.2.0
./scripts/release.sh major  # 0.1.0 -> 1.0.0
```

The release process:
1. The script runs quality checks locally
2. Creates a version bump commit and tag
3. Pushes to GitHub, triggering the CI workflow
4. After CI passes, the publish workflow automatically publishes to crates.io

## Roadmap

- [x] Create starter CLI template
- [x] Add command-line argument parsing (IP address input)
- [x] Implement ASN lookup using RIPE NCC API
- [x] Add error handling and validation
- [x] Create comprehensive tests
- [x] Support additional ASN data sources (ipapi.co)
- [ ] Add configuration file support
- [ ] Support additional data sources (team-cymru, MaxMind GeoIP2)
- [ ] Publish to crates.io

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and run tests (`cargo test && cargo clippy`)
4. Format your code (`cargo fmt`)
5. Commit your changes
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Thanks to the Rust community for excellent documentation and tools.

---

**Happy Hacking!** 🦀🌐

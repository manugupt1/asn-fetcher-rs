# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

asn-fetcher-rs is a CLI tool written in Rust that fetches Autonomous System Number (ASN) information from IP addresses using the RIPE NCC API. The project uses a trait-based architecture to allow for multiple ASN data sources in the future.

## Essential Commands

### Build and Run
```bash
# Build in development mode
cargo build

# Build optimized release binary
cargo build --release

# Run with an IP address
cargo run -- 8.8.8.8

# Run release binary directly
./target/release/asn-fetcher 8.8.8.8
```

### Testing
```bash
# Run all tests (unit and integration)
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test by name
cargo test test_name
```

### Code Quality
```bash
# Format code (required before commits)
cargo fmt

# Check formatting without changes
cargo fmt -- --check

# Run linter
cargo clippy

# Run linter treating warnings as errors
cargo clippy -- -D warnings
```

### Documentation
```bash
# Generate documentation
cargo doc

# Generate and open docs in browser
cargo doc --open
```

### Release
```bash
# Create a patch release (0.1.0 -> 0.1.1)
./scripts/release.sh patch

# Create a minor release (0.1.0 -> 0.2.0)
./scripts/release.sh minor

# Create a major release (0.1.0 -> 1.0.0)
./scripts/release.sh major
```

The release script will:
1. Run quality checks (tests, clippy, format)
2. Update version in Cargo.toml
3. Create git commit and tag
4. Push to GitHub (triggers automated publish to crates.io)

## Architecture

### Core Module Structure

```
src/
├── main.rs          # Entry point: parses CLI args, creates Ripe client, performs lookup
├── lib.rs           # Re-exports public modules
├── asn/             # ASN lookup logic
│   ├── client.rs    # Asn trait definition for lookup providers
│   ├── ripe.rs      # Ripe struct implementing Asn trait via RIPE NCC API
│   └── types.rs     # AsnInfo struct
├── cli/             # CLI argument handling
│   └── args.rs      # Args struct using clap derive
└── network/         # Network utilities (placeholder for future use)
    └── ip.rs
```

### Key Design Patterns

**Trait-Based Provider System**: The `Asn` trait in `src/asn/client.rs` defines the interface for ASN lookup providers. The `Ripe` struct implements this trait for the RIPE NCC API. This design allows adding more providers (ipapi.co, team-cymru, MaxMind) without changing the interface.

**Error Handling**: Uses `Result<T, std::io::Error>` for error propagation. The `ripe.rs` module includes `map_reqwest_error()` to convert reqwest errors to IO errors with appropriate error kinds (TimedOut, ConnectionRefused, Other).

**Blocking HTTP Client**: Currently uses `reqwest::blocking::Client` with a 10-second timeout. All API calls are synchronous.

### API Integration

The RIPE lookup (`src/asn/ripe.rs`) queries:
- URL: `https://stat.ripe.net/data/prefix-overview/data.json?resource={ip}`
- Parses JSON response extracting `data.asns[]` array
- Returns `Vec<AsnInfo>` containing ASN numbers and holder names

### Dependencies

- `clap` (v4.5.53): CLI argument parsing with derive macros
- `reqwest` (v0.13.1): HTTP client with blocking and JSON features
- `serde` (v1.0): Serialization framework with derive
- `serde_json` (v1.0.148): JSON parsing

## Testing Strategy

Unit tests are embedded in each module using `#[cfg(test)]`. Currently covers:
- Type creation and debug formatting (`types.rs`)
- CLI argument parsing (`args.rs`)
- Ripe client initialization (`ripe.rs`)

Integration tests are in `tests/integration_test.rs` but currently minimal.

#!/usr/bin/env bash
#
# Release script for asn-fetcher-rs
# Usage: ./scripts/release.sh [major|minor|patch]
#
# This script will:
# 1. Run quality checks (tests, clippy, format)
# 2. Bump the version in Cargo.toml
# 3. Create a git commit and tag
# 4. Push to GitHub (which triggers the publish workflow)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

info() {
    echo -e "${BLUE}$1${NC}"
}

success() {
    echo -e "${GREEN}$1${NC}"
}

warn() {
    echo -e "${YELLOW}$1${NC}"
}

usage() {
    echo "Usage: $0 [major|minor|patch]"
    echo ""
    echo "Bump types:"
    echo "  major - Increment major version (e.g., 0.1.0 -> 1.0.0)"
    echo "  minor - Increment minor version (e.g., 0.1.0 -> 0.2.0)"
    echo "  patch - Increment patch version (e.g., 0.1.0 -> 0.1.1)"
    exit 1
}

# Parse current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | head -n 1 | sed 's/version = "\(.*\)"/\1/'
}

# Parse version components
parse_version() {
    local version=$1
    IFS='.' read -r -a parts <<< "$version"
    MAJOR="${parts[0]}"
    MINOR="${parts[1]}"
    PATCH="${parts[2]}"
}

# Calculate new version based on bump type
bump_version() {
    local bump_type=$1
    local current_version=$2

    parse_version "$current_version"

    case $bump_type in
        major)
            MAJOR=$((MAJOR + 1))
            MINOR=0
            PATCH=0
            ;;
        minor)
            MINOR=$((MINOR + 1))
            PATCH=0
            ;;
        patch)
            PATCH=$((PATCH + 1))
            ;;
        *)
            error "Invalid bump type: $bump_type"
            ;;
    esac

    echo "${MAJOR}.${MINOR}.${PATCH}"
}

# Update version in Cargo.toml
update_cargo_toml() {
    local new_version=$1

    # Use sed to replace the version (macOS and Linux compatible)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "0,/^version = /s/^version = .*/version = \"${new_version}\"/" Cargo.toml
    else
        # Linux
        sed -i "0,/^version = /s/^version = .*/version = \"${new_version}\"/" Cargo.toml
    fi
}

# Run quality checks
run_quality_checks() {
    info "Running quality checks..."

    info "  - Running cargo test..."
    cargo test || error "Tests failed. Please fix the tests before releasing."

    info "  - Running cargo clippy..."
    cargo clippy -- -D warnings || error "Clippy checks failed. Please fix the warnings before releasing."

    info "  - Running cargo fmt --check..."
    cargo fmt -- --check || error "Format check failed. Please run 'cargo fmt' before releasing."

    success "✓ All quality checks passed!"
}

# Verify git state
verify_git_state() {
    info "Verifying git state..."

    # Check if git is available
    if ! command -v git &> /dev/null; then
        error "git is not installed"
    fi

    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        error "Not in a git repository"
    fi

    # Check if working directory is clean
    if [[ -n $(git status --porcelain) ]]; then
        error "Working directory is not clean. Please commit or stash your changes."
    fi

    # Check if remote is configured
    if ! git remote get-url origin > /dev/null 2>&1; then
        error "No 'origin' remote configured"
    fi

    # Warn if not on main branch
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$current_branch" != "main" ]]; then
        warn "Warning: You are on branch '$current_branch', not 'main'"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            error "Release cancelled"
        fi
    fi

    success "✓ Git state verified"
}

# Create release
create_release() {
    local new_version=$1
    local tag="v${new_version}"

    info "Creating release..."

    # Check if tag already exists
    if git rev-parse "$tag" >/dev/null 2>&1; then
        error "Tag $tag already exists"
    fi

    # Update Cargo.toml
    info "  - Updating Cargo.toml to version ${new_version}..."
    update_cargo_toml "$new_version"

    # Update Cargo.lock
    info "  - Updating Cargo.lock..."
    cargo check --quiet || error "Failed to update Cargo.lock"

    # Create commit
    info "  - Creating git commit..."
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to v${new_version}" || error "Failed to create commit"

    # Create tag
    info "  - Creating git tag ${tag}..."
    git tag -a "$tag" -m "Release v${new_version}" || error "Failed to create tag"

    # Push to remote
    info "  - Pushing to GitHub..."
    git push origin "$(git rev-parse --abbrev-ref HEAD)" || error "Failed to push commit"
    git push origin "$tag" || error "Failed to push tag"

    success "✓ Release created successfully!"
}

# Main script
main() {
    # Check arguments
    if [[ $# -ne 1 ]]; then
        usage
    fi

    local bump_type=$1

    # Validate bump type
    if [[ ! "$bump_type" =~ ^(major|minor|patch)$ ]]; then
        error "Invalid bump type: $bump_type"
        usage
    fi

    # Change to repository root
    cd "$(git rev-parse --show-toplevel)" || error "Failed to change to repository root"

    # Verify git state
    verify_git_state

    # Get current version
    local current_version
    current_version=$(get_current_version)
    if [[ -z "$current_version" ]]; then
        error "Could not determine current version from Cargo.toml"
    fi

    # Calculate new version
    local new_version
    new_version=$(bump_version "$bump_type" "$current_version")

    # Confirm with user
    echo ""
    info "Current version: ${current_version}"
    info "New version:     ${new_version}"
    info "Bump type:       ${bump_type}"
    echo ""
    read -p "Proceed with release? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        warn "Release cancelled"
        exit 0
    fi

    # Run quality checks
    run_quality_checks

    # Create release
    create_release "$new_version"

    # Success message
    echo ""
    success "========================================="
    success "Release v${new_version} completed!"
    success "========================================="
    echo ""
    info "Next steps:"
    info "  1. Monitor GitHub Actions: https://github.com/manugupt1/asn-fetcher-rs/actions"
    info "  2. The publish workflow will:"
    info "     - Run quality checks"
    info "     - Publish to crates.io"
    info "     - Create a GitHub Release"
    echo ""
}

# Run main function
main "$@"

# Project tasks. Run `just` to list them, or `just <recipe>` to run one.

# List the recipes
default:
    @just --list

# Build the release binary
build:
    cargo build --release

# Clean all build artifacts
clean:
    cargo clean

# Run the tests of every crate
test:
    cargo test --workspace

# Format all Rust files; `just fmt --check` only reports what is unformatted
fmt *args:
    cargo fmt --all {{args}}

# Run clippy on every crate and its tests, failing on any warning
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Build the docs, failing on any warning
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

# Check formatting, lint, test and build the docs, as CI would
ci: (fmt "--check") lint test doc

# Start the interactive prompt
repl:
    cargo run --quiet -- --repl

# Check a file and report its problems, or run it: `just run --run hello.sz`
[positional-arguments]
run *args:
    cargo run --quiet -- "$@"

# Install the pre-commit hook that formats staged Rust files
hooks:
    git config core.hooksPath .githooks

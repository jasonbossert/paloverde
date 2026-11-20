# Default recipe: list available commands
default:
    @just --list

# Regenerate .pyi type stubs from Rust
stubs:
    uv run --no-sync cargo run --bin stub_gen --no-default-features

# Rebuild the Rust extension in editable mode via uv
build:
    uv sync --reinstall-package paloverde

# Regenerate stubs and rebuild the Python extension
dev: stubs build

# Run type checking and tests
check:
    uv run basedpyright
    cargo test --workspace
    uv run pytest

# Run full pipeline: stubs -> build -> typecheck -> tests
all: dev check

# Dry-run Cargo crates.io packaging and build/verify PyPI wheel + sdist
release-dry-run:
    cargo publish --workspace --dry-run
    rm -rf dist
    uv run --with maturin maturin build --release --out dist --all-features
    uv run --with maturin maturin sdist --out dist
    uv run --with twine twine check dist/*
# paloverde

> A JIT-compiling geospatial processing engine.



`paloverde` is a geospatial processing engine that eliminates intermediate allocations and interpreter overhead by compiling GIS pipelines on the fly.


## Features
- **Fast Core**: Pure Rust implementation (`paloverde-core`).
- **Typed Python API**: Full `.pyi` type stubs included out of the box.

---

## Installation

### Python
```bash
uv add paloverde
# Or with pip:
pip install paloverde

# With optional extras:
uv add "paloverde[gpu]"
```

### Rust
```bash
cargo add paloverde

# With optional features:
cargo add paloverde -F gpu
```

---

## Development

Requires Rust, [`uv`](https://astral.sh/uv/), and [`just`](https://github.com/casey/just).

```bash
# Regenerate .pyi type stubs and build the editable Python extension
just dev
```

---

## License

Licensed under the [MIT License](LICENSE).
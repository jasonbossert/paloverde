use _paloverde as _;
use pyo3_stub_gen::{Result, StubInfo};
use std::path::Path;

fn main() -> Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let pyproject_path = manifest_dir.join("../../pyproject.toml");
    
    let stub = StubInfo::from_pyproject_toml(&pyproject_path)?;
    stub.generate()?;
    Ok(())
}
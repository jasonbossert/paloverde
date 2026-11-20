use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};

#[gen_stub_pyfunction]
#[pyfunction]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[pymodule]
fn _paloverde(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);

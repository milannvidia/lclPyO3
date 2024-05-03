use pyo3::prelude::*;
use lcl_rust::problems::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lcl_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

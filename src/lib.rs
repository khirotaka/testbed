extern crate ndarray;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::PyReadonlyArray1;


#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pyfunction]
fn add_double(a: f64, b: f64) -> PyResult<f64> {
    Ok(a + b)
}


#[pyfunction]
fn show_items1d(x: PyReadonlyArray1<f64>) {
    let x = x.as_array();
    for i in 0..x.len() {
        println!("{}", x[i])
    }
}

#[pymodule]
fn testbed(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add_double))?;
    m.add_wrapped(wrap_pyfunction!(show_items1d))?;
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;

    Ok(())
}

extern crate pyo3;
extern crate ndarray;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyArray3, IntoPyArray};
use ndarray::{Axis, Slice, Array, Array3};


#[pyclass]
struct Person {
    age: i32,
    name: String
}

#[pymethods]
impl Person {
    #[new]
    fn new(age: i32, name: String) -> Self {
        Person{ age, name }
    }

    #[getter]
    fn age(&self) -> PyResult<i32> {
        Ok(self.age)
    }

    #[getter]
    fn name(&self) -> PyResult<&String> {
        Ok(&self.name)
    }

    #[call]
    fn __call__(&self, greet: String) -> PyResult<String> {
        Ok(format!("{}! My name is {}. I'm {}.", greet, &self.name, &self.age))
    }
}

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

#[pyfunction]
fn foo(x: PyReadonlyArray2<f64>, u: usize)  {
    let x = x.as_array();
    let mut out = Array::<f64, _>::zeros((u, x.len_of(Axis(0)), x.len_of(Axis(1))));

    for i in 0..u {
        for j in 0..x.len_of(Axis(0)) {
            for k in 0..x.len_of(Axis(1)) {
                out[[i, j, k]] = x[[j, k]];
            }
        }
    }
    println!("{}", out);
}

#[pyfunction]
fn sliding_window(x: PyReadonlyArray2<f64>, window_size: usize, overlap: usize) -> PyResult<Py<PyArray3<f64>>> {
    let x = x.as_array();
    let seq_len = x.len_of(Axis(0));
    let features = x.len_of(Axis(1));
    let out_samples = (x.len_of(Axis(0)) - window_size) / overlap;

    let mut out: Array3<f64> = Array::<f64, _>::zeros((out_samples, window_size, features));

    for (idx, i) in (0 .. (seq_len - window_size)).step_by(overlap).enumerate() {
        let end = i + window_size;
        let tmp = x.slice_axis(Axis(0), Slice::from(i..end));

        for ws in 0..window_size {
            for f in 0..features {
                out[[idx, ws, f]] = tmp[[ws, f]];
            }
        }

    }
    let gil = pyo3::Python::acquire_gil();
    Ok(out.into_pyarray(gil.python()).to_owned())
}

#[pymodule]
fn testbed(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add_double))?;
    m.add_wrapped(wrap_pyfunction!(show_items1d))?;
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(sliding_window))?;
    m.add_wrapped(wrap_pyfunction!(foo))?;
    m.add_class::<Person>()?;

    Ok(())
}

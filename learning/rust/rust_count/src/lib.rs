use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PySequence;
use std::collections::HashSet;
use rand::Rng;

#[pyfunction]
#[pyo3(signature = (items, epsilon=0.5, delta=0.001))]
fn count_approx_rust(
    _py: Python<'_>,
    items: &Bound<PySequence>,
    epsilon: f64,
    delta: f64,
    ) -> PyResult<u64> {
    let mut p = 1.0;
    let mut tracked_items = HashSet::new();
    let items: Vec<String> = items
        .iter()?
        .map(|item| item.and_then(|i| i.extract::<String>()))
        .collect::<PyResult<Vec<String>>>()?;

    let max_tracked =
        ((12.0 / epsilon.powi(2)) * (8.0 * (items.len() as f64) / delta).log2()).round() as usize;
    // In future versions of PyO3 iter() will be renamed to try_iter():

    for item in items {
        if tracked_items.contains(&item) {
            continue;
        }

        if rand::thread_rng().gen::<f64>() < p {
            tracked_items.insert(item);
        }

        if tracked_items.len() == max_tracked {
            let mut temp_tracked_items = HashSet::new();
            for subitem in tracked_items {
                if rand::thread_rng().gen::<f64>() < 0.5 {
                    temp_tracked_items.insert(subitem);
                }
            }

            tracked_items = temp_tracked_items;
            p /= 2.0;

            if tracked_items.is_empty() {
                return Err(PyRuntimeError::new_err("we got unlucky, no answer"));
            }
        }
    }
    Ok((tracked_items.len() as f64 / p).round() as u64)
}

// Expose the function above via an importable Python extension.
#[pymodule]
fn rust_count(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_approx_rust, m)?)?;
    Ok(())
}

use pyo3::prelude::*;
use std::fs;
use binwalk::Binwalk;
use serde::Serialize;

#[derive(Serialize)]
struct SignatureResultJson {
    offset: usize,
    id: String,
    size: usize,
    name: String,
    confidence: u8,
    description: String,
}

#[pyfunction]
fn basic_scan(path: String) -> PyResult<String> {
    let binwalker = Binwalk::new();

    let file_data = fs::read(&path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e))
    })?;

    // Collect results into a Vec
    let mut results: Vec<SignatureResultJson> = Vec::new();
    for result in binwalker.scan(&file_data) {
        results.push(SignatureResultJson {
            offset: result.offset,
            id: result.id.to_string(),
            size: result.size,
            name: result.name.clone(),
            confidence: result.confidence,
            description: result.description.clone(),
        });
    }

    // Serialize to json
    // This needs to be loaded via json in python later on
    let json_output = serde_json::to_string_pretty(&results).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON serialization failed: {}", e))
    })?;

    Ok(json_output)
}

#[pyfuncton]
fn extract(path: String, output_path: String) -> PyResult<String> {
    /*
    Extract endpoint for binwalk

    Args:
        `path`: Path of the input file to binwalk
        `output_path`: Path of the output directory to save the results to

        Both of those are expected to be strings (else we can do `.to_string()` here just to be safe)
    */

    // Starting with reading the file
    let file_data = fs::read(&path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e))
    })?;

    let scan_results = basic_scan(&file_data);


    let binwalker = Binwalk::configure();


    let mut results; Vec<SignatureResultJson> = Vec::new();



}

/// Expose
#[pymodule]
fn _rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(basic_scan, m)?)?;
    Ok(())
}

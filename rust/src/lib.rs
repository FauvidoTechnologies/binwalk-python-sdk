use pyo3::prelude::*;
use std::fs;
use serde::Serialize;
use std::collections::HashMap;

use binwalk::Binwalk;

#[derive(Serialize)]
struct SignatureResult {
    offset: usize,
    id: String,
    size: usize,
    name: String,
    confidence: u8,
    description: String,
    extraction: Option<String>,
}

// We need this to return properly
#[derive(Serialize)]
pub struct ExtractionResultPython {
    pub size: Option<usize>,
    pub success: bool,
    pub extractor: String,
    pub do_not_recurse: bool,
    pub output_directory: String,
}
// Need to do this because the binwalk's struct is not serialisable (why?)
impl From<binwalk::extractors::common::ExtractionResult> for ExtractionResultPython {
    fn from(e: binwalk::extractors::common::ExtractionResult) -> Self {
        Self {
            size: e.size,
            success: e.success,
            extractor: e.extractor,
            do_not_recurse: e.do_not_recurse,
            output_directory: e.output_directory,
        }
    }
}

#[pyfunction]
fn scan(path: String) -> PyResult<String> {
    /*
    Function to perform scan for magic signatures.
    Returns a list of validated magic signatures representing the known contents of the file.
    
    Args:
        `path`: The path to the target binary
    */
    let binwalker = Binwalk::new();

    let file_data = fs::read(&path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e))
    })?;

    // Collect results into a Vec
    let mut results: Vec<SignatureResult> = Vec::new();
    for result in binwalker.scan(&file_data) {
        results.push(SignatureResult {
            offset: result.offset,
            id: result.id.to_string(),
            size: result.size,
            name: result.name.clone(),
            confidence: result.confidence,
            description: result.description.clone(),
            extraction: None,
        });
    }

    // Serialize to json
    // This needs to be loaded via json in python later on
    let json_output = serde_json::to_string_pretty(&results).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON serialization failed: {}", e))
    })?;

    Ok(json_output)
}

#[pyfunction]
fn extract(target_path: String, extraction_directory: String) -> PyResult<String> {
    /*
    Extract endpoint for binwalk. This extracts all extractable signatures found in a file

    Args:
        `target_path`: Path of the input file to binwalk
        `extraction_directory`: Path of the output directory to save the results to

        Both of those are expected to be strings (else we can do `.to_string()` here just to be safe)
    */

    // Starting with reading the file

    let binwalker = Binwalk::configure(Some(target_path.to_string().clone()),
                                   Some(extraction_directory.to_string().clone()),
                                   None,
                                   None,
                                   None,
                                   false
                                   ).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Binwalk config failed: {:?}", e)))?;

    let file_data = fs::read(&binwalker.base_target_file).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e))
    })?;

    // Using binwalk's own struct for this because otherwise it's gonna cry
    let file_map: Vec<_> = binwalker.scan(&file_data);
    let extraction_map = binwalker.extract(&file_data, &binwalker.base_target_file, &file_map);

    // We need to convert this back otherwise py no serialise
    let converted: HashMap<_, _> = extraction_map
        .into_iter()
        .map(|(k, v)| (k, ExtractionResultPython::from(v)))
        .collect();

    let json_output = serde_json::to_string_pretty(&converted)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON failed: {}", e)))?;

    Ok(json_output)
}

// Expose
#[pymodule]
fn _rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scan, m)?)?;
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    Ok(())
}

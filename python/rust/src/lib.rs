use pyo3::prelude::*;

#[pymodule]
/// Bindings to the Duckity Rust SDK.
mod duckity_rs {
    use pyo3::prelude::*;
    use pyo3::exceptions::PyException;

    use duckity::Challenge;

    #[pyfunction]
    #[pyo3(text_signature = "(data: bytes) -> str")]
    fn solve(data: Vec<u8>) -> PyResult<String> {
        let challenge = Challenge::decode(&data)
            .map_err(|e| PyException::new_err(format!("Failed to parse challenge: {}", e)))?;
        
        let solution = challenge.solve();

        Ok(solution.encode())
    }
}

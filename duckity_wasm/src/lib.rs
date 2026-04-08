use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process(challenge: String) -> Result<String, String> {
    let decoded = duckity_core::decode(&challenge).map_err(|e| e.to_string())?;
    let solution = duckity_core::solve(&decoded);

    duckity_core::encode(&challenge, &solution).map_err(|e| e.to_string())
}

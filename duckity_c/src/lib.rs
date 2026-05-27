use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// The status code of [`duckity_solve`].
#[repr(C)]
pub enum DuckityError {
    Ok = 0,
    NullPointer = 1,
    InvalidUtf8 = 2,
    DecodeFailed = 3,
    EncodeFailed = 4,
}

/// A wrapper over [`duckity::core`]'s functions.
/// 
/// Arguments:
/// * `challenge` - The encoded challenge string to solve.
/// * `out_solution` - A pointer to where the solution string will be written.
/// 
/// Returns:
/// [`DuckityError`] - Not zero if an error occurred.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn duckity_solve(
    challenge: *const c_char,
    out_solution: *mut *mut c_char,
) -> DuckityError {
    if challenge.is_null() || out_solution.is_null() {
        return DuckityError::NullPointer;
    }

    let challenge = match unsafe { CStr::from_ptr(challenge).to_str() } {
        Ok(v) => v,
        Err(_) => return DuckityError::InvalidUtf8,
    };

    let challenge_decoded = match duckity::core::decode(challenge) {
        Ok(v) => v,
        Err(_) => return DuckityError::DecodeFailed,
    };

    let solution = duckity::core::solve(&challenge_decoded);

    let solution_encoded = match duckity::core::encode(challenge, &solution) {
        Ok(v) => v,
        Err(_) => return DuckityError::EncodeFailed,
    };

    let c_string = match CString::new(solution_encoded) {
        Ok(v) => v,
        Err(_) => return DuckityError::EncodeFailed,
    };

    unsafe {
        *out_solution = c_string.into_raw();
    }

    DuckityError::Ok
}

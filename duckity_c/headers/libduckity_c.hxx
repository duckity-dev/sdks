#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// The status code of [`duckity_solve`].
enum class DuckityError {
  Ok = 0,
  NullPointer = 1,
  InvalidUtf8 = 2,
  DecodeFailed = 3,
  EncodeFailed = 4,
};

extern "C" {

/// A wrapper over [`duckity::core`]'s functions.
///
/// Arguments:
/// * `challenge` - The encoded challenge string to solve.
/// * `out_solution` - A pointer to where the solution string will be written.
///
/// Returns:
/// [`DuckityError`] - Not zero if an error occurred.
DuckityError duckity_solve(const char *challenge, char **out_solution);

}  // extern "C"

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # The status code of [`duckity_solve`].
  cdef enum DuckityError:
    Ok # = 0,
    NullPointer # = 1,
    InvalidUtf8 # = 2,
    DecodeFailed # = 3,
    EncodeFailed # = 4,

  # A wrapper over [`duckity::core`]'s functions.
  #
  # Arguments:
  # * `challenge` - The encoded challenge string to solve.
  # * `out_solution` - A pointer to where the solution string will be written.
  #
  # Returns:
  # [`DuckityError`] - Not zero if an error occurred.
  DuckityError duckity_solve(const char *challenge, char **out_solution);

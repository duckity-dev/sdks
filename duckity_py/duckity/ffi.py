import ctypes

from enum import Enum


ffi = ctypes.CDLL("static/libduckity_c.so")


class DuckityError(int, Enum):
    OK = 0
    NULL_POINTER = 1
    INVALID_UTF8 = 2
    DECODE_FAILED = 3
    ENCODE_FAILED = 4


class DuckityException(Exception):
    def __init__(self, error_code: DuckityError):
        self.error_code = error_code
        super().__init__(f"Error solving challenge: {error_code.name}")


ffi.duckity_solve.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_char_p)]
ffi.duckity_solve.restype = ctypes.c_int


def solve(challenge: str) -> str:
    """Decodes and solves a challenge and returns its encoded solution.

    Args:
        challenge (str): The encoded challenge string.

    Raises:
        DuckityException: An error occurred while decoding or solving the challenge.

    Returns:
        str: The encoded solution string.
    """

    solution = ctypes.c_char_p()

    result = ffi.duckity_solve(challenge.encode("utf-8"), ctypes.byref(solution))

    if result != DuckityError.OK:
        raise DuckityException(DuckityError(result))

    return solution.value.decode("utf-8")

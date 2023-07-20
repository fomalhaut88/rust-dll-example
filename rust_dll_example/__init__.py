import os
import sys
import ctypes


_dll_path = os.path.join(sys.prefix, 'dlls', 'rust_dll_example.dll')

if not os.path.exists(_dll_path):
    raise FileNotFoundError(_dll_path)

_dll = ctypes.CDLL(_dll_path)

_dll.levenshtein_distance.restype = ctypes.c_uint64
_dll.levenshtein_distance.argtypes = [
    ctypes.c_uint64, ctypes.c_char_p, ctypes.c_uint64, ctypes.c_char_p
]


def levenshtein_distance(s1, s2):
    b1 = s1.encode()
    b2 = s2.encode()
    return _dll.levenshtein_distance(len(b1), b1, len(b2), b2)

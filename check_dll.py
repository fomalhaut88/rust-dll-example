import ctypes


def check_base(dll):
    # Test add
    assert dll.add(2, 3) == 5

    # Test sqr
    dll.sqr.restype = ctypes.c_double
    dll.sqr.argtypes = [ctypes.c_double]
    assert dll.sqr(6.0) == 36.0


def check_arrays(dll):
    # Create array
    arr_type = (ctypes.c_double * 5)
    arr = arr_type(*[1.0, 2.0, 3.0, 4.0, 5.0])

    # Test array_sum
    dll.array_sum.restype = ctypes.c_double
    dll.sqr.argtypes = [ctypes.c_long, arr_type]
    assert dll.array_sum(5, arr) == 15.0

    # Test array_set
    dll.array_set.argtypes = [ctypes.c_long, arr_type, ctypes.c_double]
    dll.array_set(5, arr, ctypes.c_double(3.0))
    assert list(arr) == [3.0] * 5

    # Test array3_zero
    dll.array3_zero(arr)
    assert list(arr) == [0.0, 0.0, 0.0, 3.0, 3.0]


def check_complex(dll):
    # Complex struct
    class Complex(ctypes.Structure):
        _fields_ = [
            ('x', ctypes.c_double),
            ('y', ctypes.c_double),
        ]

        def __repr__(self):
            return f"Complex(x={self.x}, y={self.y})"

        def __eq__(self, other):
            return self.x == other.x and self.y == other.y

    z = Complex(x=3.0, y=-4.0)

    # Test complex_len
    dll.complex_len.argtypes = [Complex]
    dll.complex_len.restype = ctypes.c_double
    assert dll.complex_len(z) == 5.0

    # Test complex_conj
    dll.complex_conj.argtypes = [Complex]
    dll.complex_conj.restype = Complex
    assert dll.complex_conj(z) == Complex(x=3.0, y=4.0)

    # Test real
    dll.complex_real.restype = ctypes.c_double
    assert dll.complex_real(z) == 3.0

    # Test image
    dll.complex_image.restype = ctypes.c_double
    assert dll.complex_image(z) == -4.0

    # Test mul
    dll.complex_mul.argtypes = [ctypes.c_void_p, ctypes.c_double]
    dll.complex_mul(ctypes.byref(z), 2.0)
    assert z == Complex(x=6.0, y=-8.0)


def check_counter(dll):
    # Create a Counter instance
    dll.counter_new.restype = ctypes.c_void_p
    counter = dll.counter_new()

    # Get value
    dll.counter_get.argtypes = [ctypes.c_void_p]
    assert dll.counter_get(counter) == 0

    # Increment value
    dll.counter_increment.argtypes = [ctypes.c_void_p]
    dll.counter_increment(counter)

    # Get value
    assert dll.counter_get(counter) == 1


if __name__ == "__main__":
    dll = ctypes.CDLL("./target/release/rust_dll_example.dll")

    check_base(dll)
    check_arrays(dll)
    check_complex(dll)
    check_counter(dll)    

    print("OK")

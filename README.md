# How to compile DLL in Rust and import it in Python. Useful hints.

Here I am providing a number of examples to compile [DLL](https://en.wikipedia.org/wiki/Dynamic-link_library) in Rust. In order to check the final DLL I am using Python, although DLL standard is compatible with almost all the widely known programming languages. The examples include plain functions, input and output arrays, structures and OOP-like approach. I also attached a benchmark. In the end, I am describing a way to create a Python package powered by Rust, that is ready to distribute on [PyPI](https://pypi.org/). Source code of the project is available on https://github.com/fomalhaut88/rust-dll-example.

## How to create a DLL project in Rust

1. Create a project as a library (using the flag `--lib`): `cargo new rust-dll-example --lib`

2. Add the following section in `Cargo.toml` to specify the type of the library:

```
[lib]
crate-type = ["cdylib"]
```

3. Add `#[no_mangle]` and `extern` to the exporting functions like this:

```rust
#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
```

4. Build the project: `cargo build --release`

After that the DLL file will appear by path `./target/release/rust_dll_example.dll` if you are using Windows or something similar on different platforms (usually `.so` for Linux and `.dylib` for Mac OS X).

It is a good practice to cover your functions with standard Rust tests and benchmarks, so you can control the correctness and the performance.

### Plain functions

Here are two plain functions written in Rust:

```rust
//! An addition of two unsigned integer numbers.
#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}


//! Square of a float value.
#[no_mangle]
pub extern fn sqr(x: f64) -> f64 {
    x * x
}
```

There are two differences to normal functions: the keyword `extern` so the function is linked to DLL interface and the attribuue `no_mangle` that is needed to disable the standard name encoding on the compilation stage (read more about it here: https://doc.rust-lang.org/reference/abi.html#the-no_mangle-attribute).

To call this functions in Python, there is a following code:

```python
import ctypes

dll = ctypes.CDLL("./target/release/rust_dll_example.dll")

# Test add
assert dll.add(2, 3) == 5

# Test sqr
dll.sqr.restype = ctypes.c_double
dll.sqr.argtypes = [ctypes.c_double]
assert dll.sqr(6.0) == 36.0
```

Notice, that for `sqr` function we had to specify input and output types explicitly before the call, so Python knows how to interpret the data. It is recommended to avoid skipping types specification as it is done for `add` function, despite of usually integer data types are set by default. Do you remember the phrase `Explicit is better than implicit.` from `The Zen of Python`? (The full text of it can always be outputed by `import this`)

### Passing arrays

There are two ways to handle arrays: by reference (norammly for fixed size arrays) and by pointer (it the size is unknown or mutable from call to call), and both a supported in DLLs.

```rust
//! Sum of elements of an array given by pointer and the size.
#[no_mangle]
pub extern fn array_sum(size: usize, arr: *const f64) -> f64 {
    let mut res = 0.0;
    for idx in 0..size {
        unsafe {
            res += *arr.offset(idx as isize);
        }
    }
    res
}


//! Fill the given array with a float value.
#[no_mangle]
pub extern fn array_set(size: usize, arr: *mut f64, val: f64) {
    for idx in 0..size {
        unsafe {
            *arr.offset(idx as isize) = val;
        }
    }
}


//! Set elements of a fixed size array to zeros.
#[no_mangle]
pub extern fn array3_zero(arr: &mut [f64; 3]) {
    for idx in 0..arr.len() {
        arr[idx] = 0.0;
    }
}


//! Concatenate two arrays.
#[no_mangle]
pub extern fn array_concat(size1: usize, arr1: *const f64,
                           size2: usize, arr2: *const f64) -> *const f64 {
    let mut res = Vec::with_capacity(size1 + size2);
    res.resize(size1 + size2, 0.0);
    unsafe {
        ptr::copy(arr1, res.as_mut_ptr(), size1);
        ptr::copy(arr2, res.as_mut_ptr().add(size1), size2);
    }
    Box::new(res).as_ptr()
}


//! Return fixed size array filled with the given value.
#[no_mangle]
pub extern fn array5_fill(val: f64) -> Box<[f64; 5]> {
    Box::new([val; 5])
}
```

Accessing to the elements of an array passed by pointer is an unsafe operation for Rust, so it is reflected by the `unsafe` sections. If we return the array as the result, we can do it by reference (as it is in `array_concat`) or as a boxed fixed size array.

In order by call these functions from Python, we can use following commands:

```python
# Create array
arr_type = (ctypes.c_double * 5)
arr = arr_type(*[1.0, 2.0, 3.0, 4.0, 5.0])

# Test array_sum
dll.array_sum.restype = ctypes.c_double
dll.sqr.argtypes = [ctypes.c_uint64, arr_type]
assert dll.array_sum(5, arr) == 15.0

# Test array_set
dll.array_set.argtypes = [ctypes.c_uint64, arr_type, ctypes.c_double]
dll.array_set(5, arr, ctypes.c_double(3.0))
assert list(arr) == [3.0] * 5

# Test array3_zero
dll.array3_zero(arr)
assert list(arr) == [0.0, 0.0, 0.0, 3.0, 3.0]

# Test array_concat
dll.array_concat.argtypes = [ctypes.c_uint64, ctypes.c_double * 2, 
                             ctypes.c_uint64, ctypes.c_double * 3]
dll.array_concat.restype = ctypes.POINTER(ctypes.c_double * 5)
arr1 = (ctypes.c_double * 2)(*[1.0, 2.0])
arr2 = (ctypes.c_double * 3)(*[3.0, 4.0, 5.0])
res = dll.array_concat(2, arr1, 3, arr2)
assert list(res.contents) == [1.0, 2.0, 3.0, 4.0, 5.0]

# Test array5_fill
dll.array5_fill.argtypes = [ctypes.c_double]
dll.array5_fill.restype = ctypes.POINTER(ctypes.c_double * 5)
arr = dll.array5_fill(2.5)
assert list(arr.contents) == [2.5] * 5
```

What is interesting there? First of all, we define array data type as a product of the type of element and the size. And after that we must convert Python list to the understandable by Rust format, because Python lists are totally not the same as C-compatible arrays. Also there is no difference from Python side regarding passing by reference or by pointer.

In two last functions we return arrays as the results: as a pointer and as a fixed size array. In both cases, in Python we should set `restype` to `ctypes.POINTER`, so after that we can extract the values from `.contents` attribute.

Working with strings (or bytes) is similar, because string is represented as an array of chars (or `uint8`), so on the Rust side they should have the types `*u8`  or `[u8; SIZE]`, and on the Python side it is `ctypes.c_char * SIZE`.

### C-compatible structures

We also are allowed to work with complex data types like structures as with base types, but there are some additions. Let us look at the Rust code that implements a few functions for complex numbers:

```rust
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}


#[no_mangle]
pub extern fn complex_len(z: Complex) -> f64 {
    (z.x * z.x + z.y * z.y).sqrt()
}


#[no_mangle]
pub extern fn complex_conj(z: Complex) -> Complex {
    Complex {
        x: z.x,
        y: -z.y,
    }
}

impl Complex {
    #[no_mangle]
    #[export_name="complex_real"]
    pub extern fn real(&self) -> f64 {
        self.x
    }

    #[no_mangle]
    #[export_name="complex_image"]
    pub extern fn image(&self) -> f64 {
        self.y
    }

    #[no_mangle]
    #[export_name="complex_mul"]
    pub extern fn mul(&mut self, val: f64) {
        self.x *= val;
        self.y *= val;
    }
}
```

We can see already familiar functions with `extern` and `no_mangle` keywords and passing variables is done as it would be for standard data types. I also added an OOP-like part (`impl` section), so the functions inside can be considered as methods of the structure. They also have `extern` and `no_mandge` but there is also `export_name` attribute that customizes the name of the linked function to reach it outside.

Notice, that the structure has the attribute `repr(C)` that is important, otherwise external programs barely can understand the way the data is stored in a structure instance. This is because there are several standards to manage inner data using different alignment algorithms (you can read more about it in [Data structure alignment](https://en.wikipedia.org/wiki/Data_structure_alignment)).

As for Python code:

```python
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
```

Fortunately, `ctypes` supports the opportunity to define structure data type in a very friendly way. So after that we can work with `Complex` as with an ordinary data type, passing it to `argtypes` and `restype` attributes. But there is a peculiarity if the structure instance is supposed to be mutable: in this case we have to add `ctypes.byref` as it is done in `complex_mul` function. The argument type is set to `ctypes.c_void_p` instead of `Complex`.

This approach is good but it does not cover all the needs. The main lack is that only C-compatible data types are allowed in structure fields. If we are not going to share inner data of the structure outside there is a different approach shown in the next section.

### OOP example

In spite of the previous example, here we can use any data types in our structure we want. But we cannot access the fields outside, though we usually do not need it if we require OOP that supposes encapsulation.

```rust
struct Counter {
    val: usize,
}


impl Counter {
    #[no_mangle]
    #[export_name="counter_new"]
    pub extern fn new() -> Box<Self> {
        Box::new(Self {
            val: 0,
        })
    }

    #[no_mangle]
    #[export_name="counter_get"]
    pub extern fn get(&self) -> usize {
        self.val
    }

    #[no_mangle]
    #[export_name="counter_increment"]
    pub extern fn increment(&mut self) {
        self.val += 1;
    }
}
```

There is no `repr(C)` attribute and `new` returns boxed instance because we need to allocate the instance in the heap. The other methods are implemented similar to what we had for `Complex` structure.

```python
dll.counter_new.restype = ctypes.c_void_p
dll.counter_get.argtypes = [ctypes.c_void_p]
dll.counter_increment.argtypes = [ctypes.c_void_p]

class Counter:
    _dll = dll

    def __init__(self):
        self._counter = self._dll.counter_new()

    def get(self):
        return self._dll.counter_get(self._counter)

    def increment(self):
        self._dll.counter_increment(self._counter)

# Create an instance
counter = Counter()

# Get value
assert counter.get() == 0

# Increment value
counter.increment()

# Get value
assert counter.get() == 1
```

On the Python level, we can define `Counter` as a class with the same methods, a class object keeps the Rust counter instance inside. Notice, that `self._counter` is a pointer that has the type `ctypes.c_void_p`.

## Benchmark example

Usually, developers combine Python and a low-level programming language to improve the performance. This is the most frequent reason why DLL is needed. So this article will not be complete if I do not include a real world case with a benchmark. For this purpose I implemented a sort of [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm. I did not much care about the performance on Rust level, so it may work slower than the solutions you can find in the net, although the performance will be surely comparable.

```rust
use std::{cmp, slice};


//! Levenshtain distance algorithm
#[no_mangle]
#[export_name="levenshtein_distance"]
pub extern fn distance(n1: usize, p1: *const u8, 
                       n2: usize, p2: *const u8) -> usize {
    let s1 = unsafe { slice::from_raw_parts(p1, n1) };
    let s2 = unsafe { slice::from_raw_parts(p2, n2) };
    let lcs = lcs_solve(s1, s2);
    cmp::max(n1, n2) - lcs
}


//! Calculate Longest Common Subsequence length
pub fn lcs_solve<T: PartialEq>(s1: &[T], s2: &[T]) -> usize {
    let n1 = s1.len();
    let n2 = s2.len();

    let mut row = vec![0usize; n1];

    for i2 in 0..n2 {
        let mut prev = 0;

        for i1 in 0..n1 {
            let prev_new = row[i1];

            if s1[i1] == s2[i2] {
                row[i1] = prev + 1;
            } else if i1 > 0 {
                if row[i1] < row[i1 - 1] {
                    row[i1] = row[i1 - 1];
                }
            }

            prev = prev_new;
        }
    }

    row[n1 - 1]
}
```

Also I attached Rust benchmark:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_distance(bencher: &mut Bencher) {
        let s1 = "lewenstein";
        let s2 = "levenshtein";

        bencher.iter(|| {
            distance(s1.len(), s1.as_ptr(), 
                     s2.len(), s2.as_ptr());
        });
    }
}
```

Before run the benchmark do not forget to switch to the [Nightly toolchain](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html):

```
rustup default nightly
```

On Python level we have following script (file `rust_dll_example/__init__.py`):

```python
import ctypes


_dll = ctypes.CDLL("./target/release/rust_dll_example.dll")

_dll.levenshtein_distance.restype = ctypes.c_uint64
_dll.levenshtein_distance.argtypes = [
    ctypes.c_uint64, ctypes.c_char_p, ctypes.c_uint64, ctypes.c_char_p
]


def levenshtein_distance(s1, s2):
    b1 = s1.encode()
    b2 = s2.encode()
    return _dll.levenshtein_distance(len(b1), b1, len(b2), b2)
```

If you want to benchmark the result on Python level, you can always do it with the help of [timeit](https://docs.python.org/3/library/timeit.html) with the command:

```
python -m timeit -s 'from rust_dll_example import levenshtein_distance' 'levenshtein_distance(\"lewenstein\", \"levenshtein\")'
```

## How to prepare a Python package

If we are going to distribute the Python library with compiled DLL inside, first, it is necessary to prepare `setup.py` having some non-standard lines. Here is an example:

```python
import os
import json
import subprocess as sp

from setuptools import find_packages, setup


def build_src():
    sp.Popen(["cargo", "build", "--release"]).communicate()


def get_version():
    if os.path.exists('version'):
        with open('version') as f:
            return f.read()

    else:
        out, _ = sp.Popen(["cargo", "metadata"], stdout=sp.PIPE).communicate()
        metadata = json.loads(out.decode())
        version = metadata['packages'][0]['version']

        with open('version', 'w') as f:
            f.write(version)

        return version


def get_long_description():
    with open('README.md') as f:
        return f.read()


def get_dll_paths():
    return [
        './target/release/rust_dll_example.dll',
    ]


# Build from source
build_src()


# Setup
setup(
    name='rust-dll-example',
    version=get_version(),
    packages=find_packages(),
    license="MIT",
    description="",
    long_description=get_long_description(),
    long_description_content_type="text/markdown",
    install_requires=[],
    data_files=[('dlls', get_dll_paths()), ('', ['version'])],
)
```

Here we specified the path to DLL file as a data file for our package. Obviously, the old path `./target/release/rust_dll_example.dll` will not work. So we need to make some changes in `__init__.py` to import DLL correctly:

```python
...
_dll_path = os.path.join(sys.prefix, 'dlls', 'rust_dll_example.dll')
_dll = ctypes.CDLL(_dll_path)
...
```

Once `setup.py` is created and the path to DLL is corrected, we can build our Python package:

```
python setup.py sdist
```

After that the file `dist/rust-dll-example-0.1.0.tar.gz` will appear. It can be installed with `pip` and distributed on [PyPI](https://pypi.org/). In order to upload it on PyPI, run the following command:

```
twine upload dist/rust-dll-example-0.1.1.tar.gz
```

After that the project will be available on https://pypi.org/project/rust-dll-example/.

Notice! As far as DLL is compiled under a single platform (Windows 11 in my case), the library can be successfully installed only for the users with the same platform. Otherwise, the attached DLL file cannot be executed correctly. If you would like your library to be crossplatform, you need to build several DLLs for each platform and implement a more tricky way to catch the right DLL on Python level.

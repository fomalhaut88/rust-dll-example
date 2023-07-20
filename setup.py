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

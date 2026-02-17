#!/usr/bin/env python3
"""
Call ffi_demo shared library from Python using ctypes.

Usage:
1) Build the shared library:
   cargo build --release -p ffi_demo
2) Run this script:
   python crates/ffi_demo/examples/call_from_python.py
"""

from ctypes import CDLL, c_int32, c_uint32, c_uint64
from pathlib import Path
import platform


def shared_lib_name() -> str:
    system = platform.system().lower()
    if system == "windows":
        return "ffi_demo.dll"
    if system == "darwin":
        return "libffi_demo.dylib"
    return "libffi_demo.so"


def main() -> None:
    repo_root = Path(__file__).resolve().parents[3]
    lib_path = repo_root / "target" / "release" / shared_lib_name()

    if not lib_path.exists():
        raise FileNotFoundError(
            f"Could not find shared library at {lib_path}. "
            "Run: cargo build --release -p ffi_demo"
        )

    lib = CDLL(str(lib_path))

    lib.add.argtypes = [c_int32, c_int32]
    lib.add.restype = c_int32

    lib.fibonacci.argtypes = [c_uint32]
    lib.fibonacci.restype = c_uint64

    print(f"Loaded: {lib_path}")
    print(f"add(20, 22) = {lib.add(20, 22)}")
    print(f"fibonacci(40) = {lib.fibonacci(40)}")


if __name__ == "__main__":
    main()

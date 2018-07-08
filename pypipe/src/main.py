#!/usr/bin/env python3
# coding: utf-8

# LD_LIBRARY_PATH='.'
import ctypes

RUST_LIB = "libcallerfunctions.so"
# class List_4(ctypes.Structure):
#     # Similar to creating the struct in Rust
#     _fields_ = [("array", ctypes.ARRAY(ctypes. , 4))]


def rust_test():
    list_to_send = ['edges.txt', 'vertices.txt', 'feats.txt']
    lib = ctypes.cdll.LoadLibrary("../../target/release/libpypipe.so")
    # print(list_to_send * len(list_to_send))
    # quit()

    # c_array = (ctypes.c_char_p * len(list_to_send))(*list_to_send)
    # lib.call_from_python(c_array, len(list_to_send))
    print(lib.test_ffi(1))
    # print("file list = {}".format(libcallerfunctions.send_file_list(files)))


if __name__ == "__main__":
    rust_test()

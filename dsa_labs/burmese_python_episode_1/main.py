# import ctypes
# import timeit
# import os

# # Load the DLL
# dll_path = os.path.abspath("dsa_wasm/target/release/dsa_native.dll")
# lib = ctypes.CDLL(dll_path)

# # Set the argument type to unsigned 64-bit integer
# lib.bench_internal.argtypes = [ctypes.c_uint64]

# def run_final_boss():
#     iterations = 1_000_000_00 # The 1 Billion mark
    
#     print(f"🚀 Launching the Billion-Node Strike...")
#     start = timeit.default_timer()
    
#     # Call Rust directly
#     lib.bench_internal(iterations)
    
#     end = timeit.default_timer()
#     print(f"🦀 Rust Native (DLL): {end - start:.4f}s")

# if __name__ == "__main__":
#     run_final_boss()


import timeit
from dsa_kuuking.linked_lists.linked_list.implementation.linked_list import LinkedList
from wasmtime import Store, Module, Instance, Engine

def run_bench():
    engine = Engine()
    store = Store(engine)
    # Point to the NEW raw wasm file
    module = Module.from_file(engine, 'dsa_wasm/target/wasm32-unknown-unknown/release/dsa_wasm.wasm')
    instance = Instance(store, module, [])
    
    exports = instance.exports(store)
    # rust_insert = exports["insert_val"]
    # rust_remove = exports["remove_front"]
    rust_bench_internal = exports["bench_internal"]

    iterations = 1_000_000_00 # The "Phoenix Cluster" Scale

    # Python List
    py_ll = LinkedList[int]()
    start_py = timeit.default_timer()
    for i in range(iterations):
        py_ll.add_to_front(i)
    for _ in range(iterations):
        py_ll.remove_head()
    end_py = timeit.default_timer()

    # Rust: WASM
    start_rs = timeit.default_timer()
    rust_bench_internal(store, iterations)
    end_rs = timeit.default_timer()

    print(f"🐍 Python 3.14: {end_py - start_py:.4f}s")
    print(f"🦀 Rust WASM:   {end_rs - start_rs:.4f}s")
    print(f"\nResult: Rust is {(end_py-start_py)/(end_rs-start_rs):.1f}x faster.")

if __name__ == "__main__":
    run_bench()
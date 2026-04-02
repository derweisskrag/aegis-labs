import timeit
# from dsa_kuuking.queues.queue.implementation.queue_list import Queue
from wasmtime import Store, Module, Instance, Engine
from collections import deque

from dsa_kuuking.types.ResultType import ResultType
from dsa_kuuking.types.errors.empty_queue_exception import EmptyQueueException
from dsa_kuuking.types.errors.size_management_exception import SizeManagementException
from dsa_kuuking.interfaces.queue_interface import QueueInterface
from dsa_kuuking.linked_lists.doubly_linked_list.implementation.doubly_linked_list import DoublyLinkedList
from typing import override, List, Deque, Set


class Queue[T](QueueInterface):
    def __init__(self):
        self.count = 0
        self._hash_set: Set[T] = set()
        # Changed from List to Deque
        # self._queue: deque[T] = deque() 
        self._queue: Deque[T] = deque()
    @override
    def enqueue(self, element: T) -> None:
        result = self.manage_size(action="enqueue", value=element)

        if result.is_ok():
            # append() is O(1) for deque
            self._queue.append(element) 
        else:
            raise Exception(result.unwrap_err())

    @override
    def dequeue(self) -> ResultType[T, str]:
        peek_result = self.get_peek()

        if peek_result.is_ok():
            result = self.manage_size(action="dequeue", value=peek_result.unwrap())

            if result.is_ok():
                # popleft() is O(1) for deque. Shifting is gone!
                # Returned to pop(o) to check for Python speed
                return ResultType(value=self._queue.popleft()) 
            else:
                raise SizeManagementException(result.unwrap_err())
        else:
            raise EmptyQueueException(peek_result.unwrap_err())

    @override
    def get_peek(self) -> ResultType[T, str]:
        if self.is_empty():
            return ResultType(error="Cannot retrieve the peek from empty queue!")

        # O(1) access
        peek_value = self._queue[0] 

        if peek_value is None:
            return ResultType(error="Peek value is not defined")

        return ResultType(value=peek_value)

    @override
    def peek(self) -> None:
        # try to get peek
        result = self.get_peek()

        if result.is_ok():
            print("Peek is ", result.unwrap())
        else:
            raise Exception(result.unwrap_err())

    @override
    def is_empty(self) -> bool:
        return self.count == 0

    @override
    def traverse(self) -> Iterable[T]:
        for element in self._queue:
            yield element

    @override
    def display(self) -> None:
        if self.is_empty():
            print("Queue is empty: Queue()")

        print(self._queue)

    @override
    def manage_size(self, action: str = "", value: T = None) -> ResultType[str, str]:
        match action:
            case "enqueue":
                self.count += 1
                self._hash_set.add(value)
                return ResultType(value="Success! The value has been enqueued!")
            case "dequeue":
                self.count -= 1
                self._hash_set.remove(value)
                return ResultType(value="Success! The value has been dequeued!")
            case _:
                return ResultType(error="Wrong action!")

    def __contains__(self, element: T) -> bool:
        return element in self._hash_set

    def __str__(self) -> str:
        return "->".join(map(str, self._queue))

    def __len__(self) -> int:
        return self.count
    

class LinkedListQueue[T](QueueInterface[T]):
    def __init__(self):
        # Assuming your SinglyLinkedList has a .head and .tail
        self._list = DoublyLinkedList[T]()
        self.count = 0

    @override
    def enqueue(self, element: T) -> None:
        # We add to the back (tail) -> O(1)
        self._list.add_to_end(element) 
        self.count += 1

    @override
    def dequeue(self) -> ResultType[T, str]:
        if self.is_empty():
            raise EmptyQueueException("Queue is empty!")
            
        # We remove from the front (head) -> O(1)
        value = self._list.remove_head() 
        self.count -= 1
        return ResultType(value=value)

    @override
    def is_empty(self) -> bool:
        return self.count == 0
    
    @override
    def get_peek(self) -> ResultType[T, str]:
        return ResultType(value=self._list.get_head().unwrap())
    
    @override
    def peek(self) -> None:
        result = self.get_peek()

        if result.is_ok():
            print("The (front) peek is ", result.unwrap())
        else:
            print(f"An error occurred: {result.unwrap_err()}")

    @override
    def traverse(self) -> Iterable[T]:
        for element in self._list.traverse():
            yield element

    @override
    def display(self) -> None:
        self._list.display()

    @override
    def manage_size(self, action: str) -> ResultType[str, str]:
        raise NotImplemented("This method is not intended")


def run_bench():
    engine = Engine()
    store = Store(engine)
    # iterations = 100_000_000 # 100M f32 is roughly 400MB-800MB depending on overhead
    # iterations = 100000000
    iterations = 10000000


    # --- ZIG SETUP ---
    with open('zig_wasm/queue_zig.wasm', 'rb') as f:
        zig_bytes = f.read()
        if zig_bytes[:4] != b'\x00asm':
            print("Error: Zig file is NOT a valid WASM binary!")

    zig_module = Module(engine, zig_bytes)
    zig_instance = Instance(store, zig_module, [])
    zig_bench = zig_instance.exports(store)["bench_internal"]

    # --- RUST SETUP ---
    rs_module = Module.from_file(engine, 'bench_vec_deque/bench_vec_deque.wasm')
    rs_instance = Instance(store, rs_module, [])
    rs_bench = rs_instance.exports(store)["bench_internal"]

    # 1. Python List (Do this first or last to manage RAM)
    py_queue = LinkedListQueue[int]()
    start_py = timeit.default_timer()
    for i in range(iterations):
        py_queue.enqueue(i)
    for _ in range(iterations):
        py_queue.dequeue()
    end_py = timeit.default_timer()
    
    # Cleanup Python memory before WASM starts
    del py_queue 
    py_time = end_py - start_py
    print(f"🐍 Python 3.14 (DoublyLinkedList Queue): {py_time:.4f}s")

    # DEQUE
    py_deque = Queue[int]()
    start_py_deq = timeit.default_timer()
    for i in range(iterations):
        py_deque.enqueue(i)
    for _ in range(iterations):
        py_deque.dequeue()
    end_py_deq = timeit.default_timer()

    # Clean Up BOTH AGAIN!
    del py_deque 
    py_time_deq = end_py_deq - start_py_deq
    print(f"🐍 Python 3.14 (DEQUE): {py_time_deq:.4f}s")


    # 2. Rust: WASM
    start_rs = timeit.default_timer()
    rs_bench(store, iterations)
    end_rs = timeit.default_timer()
    rs_time = end_rs - start_rs
    print(f"🦀 Rust WASM:   {rs_time:.4f}s")

    # 3. Zig: WASM
    start_zig = timeit.default_timer()
    zig_bench(store, float(iterations)) # Note: float() because we used f32 in Zig
    end_zig = timeit.default_timer()
    zig_time = end_zig - start_zig
    print(f"⚡ Zig WASM:    {zig_time:.4f}s")

    # Results
    print(f"\n--- Final Results ---")
    print(f"Rust is {py_time/rs_time:.1f}x faster than Python Queue by DSA_KUUKING")
    print(f"Zig is {py_time/zig_time:.1f}x faster than Python Queue by DSA_KUUKING")
    print(f"Rust is {py_time_deq/rs_time:.1f}x faster than Python DEQUE")
    print(f"Zig is {py_time_deq/zig_time:.1f}x faster than Python DEQUE")

if __name__ == "__main__":
    run_bench()
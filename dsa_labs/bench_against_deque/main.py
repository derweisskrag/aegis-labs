import time, gc, statistics
from collections import deque
from dsa_kuuking.linked_lists.doubly_linked_list.implementation.doubly_linked_list import DoublyLinkedList


class Node:
    __slots__ = ['data', 'next', 'prev']  # No more __dict__ overhead!
    def __init__(self, val):
        self.data = val
        self.next = None
        self.prev = None

class DoublyLinkedListKuuking:
    def __init__(self):
        self._head = None
        self._tail = None
        self.size = 0

    def add_to_end(self, val):
        new_node = Node(val)
        if not self._tail:
            self._head = self._tail = new_node
        else:
            new_node.prev = self._tail
            self._tail.next = new_node
            self._tail = new_node
        self.size += 1

    def remove_head(self):
        if not self._head:
            return None
        
        removed = self._head
        self._head = self._head.next
        
        if self._head:
            self._head.prev = None
        else:
            self._tail = None # List is now empty
            
        self.size -= 1
        return removed

    def remove_tail(self):
        if not self._tail:
            return None
            
        node_to_remove = self._tail
        self._tail = self._tail.prev
        
        if self._tail:
            self._tail.next = None
        else:
            self._head = None # List is now empty
            
        self.size -= 1
        return node_to_remove
    
class QueueFacade:
    def __init__(self, thresholds=(1_000_000, 10_000_000)):
        self._py = LinkedListQueue()
        self._wasm_medium = None  # Zig instance
        self._wasm_large = None   # Rust instance
        self.backend = "py"
        self.thresholds = thresholds  # (zig_threshold, rust_threshold)

    def enqueue(self, item):
        if self.backend == "py":
            self._py.enqueue(item)
            self._check_migrate_after_enqueue()
        elif self.backend == "zig":
            self._wasm_medium.bulk_push([item])  # prefer bulk APIs
        else:  # rust
            self._wasm_large.bulk_push([item])

    def dequeue(self):
        if self.backend == "py":
            return self._py.dequeue()
        elif self.backend == "zig":
            return self._wasm_medium.bulk_pop(1)[0]
        else:
            return self._wasm_large.bulk_pop(1)[0]

    def __len__(self):
        if self.backend == "py":
            return len(self._py)
        return self._current_wasm_len()

    def _check_migrate_after_enqueue(self):
        n = len(self._py)
        z_th, r_th = self.thresholds
        if n >= r_th:
            self._migrate_to("rust")
        elif n >= z_th:
            self._migrate_to("zig")

    def _migrate_to(self, target):
        if self.backend == target:
            return
        # 1) export a contiguous representation of elements
        data = list(self._py.traverse())  # produces Python list of items
        # 2) create / init target wasm backend if needed
        if target == "zig":
            if self._wasm_medium is None:
                self._wasm_medium = load_zig_wasm(...)
            self._wasm_medium.bulk_push(data)  # one call that copies array into wasm
        elif target == "rust":
            if self._wasm_large is None:
                self._wasm_large = load_rust_wasm(...)
            self._wasm_large.bulk_push(data)
        # 3) swap backend and drop Python queue
        self._py = None
        self.backend = target
    

def bench_deque(n):
    q = deque()
    ap = q.append
    pl = q.popleft
    for i in range(n):
        ap(i)
    for _ in range(n):
        pl()

def bench_linkedlist(n):
    ll = DoublyLinkedList()
    add = ll.add_to_end
    remove = ll.remove_head
    for i in range(n):
        add(i)
    for _ in range(n):
        remove()

def bench_standard_linked_list(n):
    dll = DoublyLinkedListKuuking()
    add = dll.add_to_end
    remove = dll.remove_head
    for i in range(n):
        add(i)
    for _ in range(n):
        remove()

def time_runs(func, n, runs=5):
    times = []
    for _ in range(runs):
        gc.collect()
        start = time.perf_counter()
        func(n)
        times.append(time.perf_counter() - start)
    return times

if __name__ == "__main__":
    N = 1_000_000          # start smaller, increase as you can
    runs = 5

    dq_times = time_runs(bench_deque, N, runs)
    ll_times = time_runs(bench_linkedlist, N, runs)
    dll_times = time_runs(bench_standard_linked_list, N, runs)

    print("deque:", dq_times, "median:", statistics.median(dq_times), "stdev:", statistics.pstdev(dq_times))
    print("linkedlist:", ll_times, "median:", statistics.median(ll_times), "stdev:", statistics.pstdev(ll_times))
    print("DLL (simplified):", dll_times, "median:", statistics.median(dll_times), "stdev:", statistics.pstdev(dll_times))
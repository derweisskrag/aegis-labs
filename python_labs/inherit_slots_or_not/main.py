from typing import Optional

class LinkedListNode[T]:
    """A node in a linked list."""
    __slots__ = ['_data', 'next', '_id']
    def __init__(self, data: T):
        self._data = data
        self.next: Optional['LinkedListNode[T]'] = None
        self._id = id(self)

    @property
    def data(self) -> T:
        """Getter method for the data."""
        return self._data

    def get_id(self) -> int:
        """Retrieves the id of a node"""
        return self._id

    def __repr__(self):
        return f"Node(data={self.data})"


class DoublyLinkedListNode[T](LinkedListNode):
    """A node in a doubly linked list."""
    __slots__ = ['prev']
    def __init__(self, data: T) -> None:
        super().__init__(data)
        self.prev: Optional['DoublyLinkedListNode[T]'] = None

if __name__ == "__main__":
    # Example usage
    node1 = LinkedListNode(1)
    node2 = LinkedListNode(2)
    node3 = LinkedListNode(3)

    node1.next = node2
    node2.next = node3

    print(node1)  # Output: Node(data=1)
    print(node1.next)  # Output: Node(data=2)
    print(node1.next.next)  # Output: Node(data=3)

    dnode1 = DoublyLinkedListNode(10)
    dnode2 = DoublyLinkedListNode(20)

    dnode1.next = dnode2
    dnode2.prev = dnode1

    print(dnode1)  # Output: Node(data=10)
    print(dnode1.next)  # Output: Node(data=20)
    print(dnode2.prev)  # Output: Node(data=10)

    print("Check if slots work")
    # print(node1.__dict__)  # This will raise an AttributeError because of __slots__
    print(dnode1.__dir__())  # This will show the attributes defined in __slots__
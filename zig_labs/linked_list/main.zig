const std = @import("std");

/// A single node in a singly-linked list.
const Node = struct {
    value: i32,
    next: ?*Node,
};

/// A simple singly-linked list implementation.
/// It supports appending values and printing the list.
/// Memory is managed using the provided allocator. 
const LinkedList = struct {
    head: ?*Node,
    allocator: std.mem.Allocator,

    /// Initializes a new linked list with the given allocator.
    pub fn init(allocator: std.mem.Allocator) LinkedList {
        return LinkedList{ .head = null, .allocator = allocator };
    }


    /// Appends a new value to the end of the linked list.
    /// Allocates memory for a new node and updates the list accordingly.
    /// Errors during allocation are propagated to the caller.
    pub fn append(self: *LinkedList, value: i32) !void {
        const newNode = try self.allocator.create(Node);
        newNode.* = Node{ .value = value, .next = null };

        if (self.head == null) {
            self.head = newNode;
        } else {
            var current = self.head;
            while (current) |node| {
                if (node.next == null) {
                    node.next = newNode;
                    return;
                }
                current = node.next;
            }
        }
    }


    /// Prints the values in the linked list to the standard output.
    pub fn print(self: *LinkedList) void {
        var current = self.head;
        while (current) |node| {
            std.debug.print("{} ", .{node.value});
            current = node.next;
        }
        std.debug.print("\n", .{});
    }


    /// Deinitializes the linked list by freeing all allocated nodes.
    /// After calling this, the list becomes empty.
    pub fn deinit(self: *LinkedList) void {
        var current = self.head;
        while (current) |node| {
            const next = node.next;
            self.allocator.destroy(node);
            current = next;
        }
        // Set head to null to indicate the list is now empty.
        self.head = null;
    }

};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var list = LinkedList.init(allocator);
    defer list.deinit();

    try list.append(1);
    try list.append(2);
    try list.append(3);
    list.print();
}


test "linked list append" {
    const gpa = std.testing.allocator;
    var list = LinkedList.init(gpa);
    defer list.deinit();

    try list.append(10);
    try list.append(20);
    try list.append(30);

    const head = list.head.?;
    try std.testing.expectEqual(10, head.value);

    const second = head.next.?;
    try std.testing.expectEqual(20, second.value);

    const third = second.next.?;
    try std.testing.expectEqual(30, third.value);

    try std.testing.expect(third.next == null);
}


const std = @import("std");

export fn bench_internal(iters: f32) void {
    // We use PageAllocator because it talks directly to WASM memory pages
    const allocator = std.heap.page_allocator;

    var fifo = std.fifo.LinearFifo(f32, .Dynamic).init(allocator);
    defer fifo.deinit();

    var i: f32 = 0;
    while (i < iters) : (i += 1) {
        // 'catch {}' is used because we're in a benchmark and 
        // assuming memory allocation won't fail for this test.
        fifo.writeItem(i) catch {};
    }

    while (fifo.readItem()) |_| {}
}
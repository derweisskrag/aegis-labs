const std = @import("std");

// Step 1: comptime calculation
const n: u16 = 100;
const sum: u16 = (n * (n + 1)) / 2;

pub fn main() void {
    // Step 2: runtime printing
    std.debug.print(
        "The sum of the first {d} natural numbers is: {d}\n",
        .{ n, sum },
    );
}

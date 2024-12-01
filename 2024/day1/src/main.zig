const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var list1 = std.ArrayList(i32).init(allocator);
    defer list1.deinit();
    var list2 = std.ArrayList(i32).init(allocator);
    defer list2.deinit();

    var lines = std.mem.tokenize(u8, @embedFile("input.txt"), "\n");
    while (lines.next()) |line| {
        var numbers = std.mem.tokenize(u8, line, " ");
        try list1.append(try std.fmt.parseInt(i32, numbers.next().?, 10));
        try list2.append(try std.fmt.parseInt(i32, numbers.next().?, 10));
    }

    std.mem.sort(i32, list1.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, list2.items, {}, comptime std.sort.asc(i32));

    var total_diff: u32 = 0;
    for (list1.items, list2.items) |num1, num2| {
        total_diff += @abs(num1 - num2);
    }

    std.debug.print("[PART 1] sum of distances: {}\n", .{total_diff});

    var frequency = std.AutoHashMap(i32, u32).init(allocator);
    defer frequency.deinit();

    for (list2.items) |num| {
        const entry = try frequency.getOrPutValue(num, 0);
        entry.value_ptr.* += 1;
    }

    var total_score: u32 = 0;
    for (list1.items) |num| {
        const numU32: u32 = @intCast(num);
        const freq = frequency.get(num) orelse 0;
        total_score += numU32 * freq;
    }

    std.debug.print("[PART 2] total score: {}\n", .{total_score});
}

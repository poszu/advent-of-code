const std = @import("std");

fn checkSequence(numbers: []const i64) ?usize {
    if (numbers.len < 2) return null;

    var increasing: ?bool = null;

    for (0..numbers.len - 1) |index| {
        const current, const next = .{ numbers[index], numbers[index + 1] };
        const diff = current - next;
        if (diff == 0 or @abs(diff) > 3) return index;

        if (increasing) |inc| {
            if (inc != (diff > 0)) return index;
        } else {
            increasing = diff > 0;
        }
    }
    return null;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    var lines = std.mem.tokenize(u8, @embedFile("input.txt"), "\n");
    var safe: u64 = 0;
    var safe_fixed: u64 = 0;

    // Temporary array to store numbers from each line
    var numbers = std.ArrayList(i64).init(std.heap.page_allocator);
    defer numbers.deinit();

    while (lines.next()) |line| {
        // Parse all numbers from the line
        numbers.clearRetainingCapacity();
        var num_tokens = std.mem.tokenize(u8, line, " ");
        while (num_tokens.next()) |num_str| {
            try numbers.append(try std.fmt.parseInt(i64, num_str, 10));
        }

        var invalid_idx: usize = 0;
        if (checkSequence(numbers.items)) |idx| {
            invalid_idx = idx;
        } else {
            safe += 1;
            continue;
        }

        var temp = try std.ArrayList(i64).initCapacity(std.heap.page_allocator, numbers.items.len);
        defer temp.deinit();

        const start = if (invalid_idx > 0)
            invalid_idx - 1
        else
            invalid_idx;

        for (start..start + 3) |skip_idx| {
            temp.clearRetainingCapacity();
            for (numbers.items, 0..) |num, i| {
                if (i != skip_idx) {
                    try temp.append(num);
                }
            }

            if (checkSequence(temp.items) == null) {
                safe_fixed += 1;
                break;
            }
        }
    }

    try stdout.print("[PART 1] {d}\n", .{safe});
    try stdout.print("[PART 2] {d}\n", .{safe_fixed + safe});
}

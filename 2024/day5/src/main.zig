const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    const total = try solve2(input);
    try stdout.print("[PART 1] total = {}\n", .{total[0]});
    try stdout.print("[PART 2] total = {}\n", .{total[1]});
}

fn solve2(input: []const u8) ![2]usize {
    const allocator = std.heap.page_allocator;
    var ordering_rules = std.AutoHashMap(usize, std.AutoHashMap(usize, void)).init(allocator);
    defer {
        var it = ordering_rules.valueIterator();
        while (it.next()) |inner_map| {
            inner_map.deinit();
        }
        ordering_rules.deinit();
    }
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |l| {
        if (l.len == 0) {
            break;
        }
        var split = std.mem.splitScalar(u8, l, '|');
        const num1 = try std.fmt.parseInt(usize, split.next().?, 10);
        const num2 = try std.fmt.parseInt(usize, split.next().?, 10);
        var entry = try ordering_rules.getOrPutValue(num1, std.AutoHashMap(usize, void).init(allocator));
        try entry.value_ptr.put(num2, {});
    }

    var total1: usize = 0;
    var total2: usize = 0;
    while (lines.next()) |l| {
        if (l.len == 0) {
            break;
        }
        var updates = std.ArrayList(usize).init(allocator);
        defer updates.deinit();
        var numbers = std.mem.splitScalar(u8, l, ',');
        while (numbers.next()) |num_str| {
            const num = try std.fmt.parseInt(usize, num_str, 10);
            try updates.append(num);
        }

        const wasCorrect = try processUpdates(&updates, &ordering_rules);
        const middle = updates.items[updates.items.len / 2];
        if (wasCorrect) {
            total1 += middle;
        } else {
            total2 += middle;
        }
    }
    return .{ total1, total2 };
}

fn processUpdates(updates: *std.ArrayList(usize), rules: *std.AutoHashMap(usize, std.AutoHashMap(usize, void))) !bool {
    var wasCorrrect = true;
    for (0.., updates.items) |i, num| {
        const must_after = rules.getPtr(num) orelse {
            continue;
        };
        for (0..i) |pos| {
            if (must_after.contains(updates.items[pos])) {
                // put `num` in front of `value`
                wasCorrrect = false;
                _ = updates.orderedRemove(i);
                try updates.insert(pos, num);
                break;
            }
        }
    }
    return wasCorrrect;
}

test "check line" {
    const input =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;

    const solution = try solve2(input);
    try std.testing.expectEqual(143, solution[0]);
    try std.testing.expectEqual(123, solution[1]);
}

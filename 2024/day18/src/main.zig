const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");
    const bytes = try parseInput(input, std.heap.page_allocator);
    defer bytes.deinit();

    const result = try solvePart1(70, bytes.items[0..1024], std.heap.page_allocator);
    std.debug.print("PART 1: {?}\n", .{result});

    const coords = try solvePart2(70, bytes.items, std.heap.page_allocator);
    std.debug.print("PART 2: {},{}\n", .{ coords.x, coords.y });
}

const Vec2 = struct {
    x: usize,
    y: usize,
};

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Vec2) {
    var bytes = std.ArrayList(Vec2).init(allocator);
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var it = std.mem.splitScalar(u8, line, ',');
        const x = try std.fmt.parseInt(usize, it.next().?, 10);
        const y = try std.fmt.parseInt(usize, it.next().?, 10);
        try bytes.append(Vec2{ .x = x, .y = y });
    }

    return bytes;
}

test "part 1" {
    const input =
        \\5,4
        \\4,2
        \\4,5
        \\3,0
        \\2,1
        \\6,3
        \\2,4
        \\1,5
        \\0,6
        \\3,3
        \\2,6
        \\5,1
        \\1,2
        \\5,5
        \\2,5
        \\6,5
        \\1,4
        \\0,4
        \\6,4
        \\1,1
        \\6,1
        \\1,0
        \\0,5
        \\1,6
        \\2,0
    ;
    const bytes = try parseInput(input, std.testing.allocator);
    defer bytes.deinit();

    const result = try solvePart1(6, bytes.items[0..12], std.testing.allocator);
    try std.testing.expectEqual(22, result);

    const part2 = try solvePart2(6, bytes.items, std.testing.allocator);
    try std.testing.expectEqual(Vec2{ .x = 6, .y = 1 }, part2);
}

fn solvePart1(comptime size: usize, bytes: []const Vec2, allocator: std.mem.Allocator) !?usize {
    var memory: [size + 1][size + 1]bool = std.mem.zeroes([size + 1][size + 1]bool);
    for (bytes) |pos| {
        memory[pos.y][pos.x] = true;
    }

    var queue = std.ArrayList(Step).init(allocator);
    defer queue.deinit();
    try queue.append(Step{ .cost = 0, .pos = Vec2{ .x = 0, .y = 0 } });

    // lowest cost to reach position
    var lowest_cost = std.AutoHashMap(Vec2, usize).init(allocator);
    defer lowest_cost.deinit();

    var best_path: ?usize = null;

    const end_position = Vec2{ .x = size, .y = size };

    while (queue.pop()) |step| {
        if (std.meta.eql(step.pos, end_position)) {
            best_path = @min(best_path orelse std.math.maxInt(usize), step.cost);
        }
        if (best_path != null and step.cost >= best_path.?) continue;

        const lowest_cost_to_reach_pos = try lowest_cost.getOrPutValue(step.pos, std.math.maxInt(usize));
        if (step.cost >= lowest_cost_to_reach_pos.value_ptr.*) continue;
        try lowest_cost.put(step.pos, step.cost);

        if (step.pos.x > 0) {
            const next_pos = Vec2{ .x = step.pos.x - 1, .y = step.pos.y };
            if (!memory[next_pos.y][next_pos.x]) {
                try queue.append(Step{ .cost = step.cost + 1, .pos = next_pos });
            }
        }
        if (step.pos.x < size) {
            const next_pos = Vec2{ .x = step.pos.x + 1, .y = step.pos.y };
            if (!memory[next_pos.y][next_pos.x]) {
                try queue.append(Step{ .cost = step.cost + 1, .pos = next_pos });
            }
        }
        if (step.pos.y > 0) {
            const next_pos = Vec2{ .x = step.pos.x, .y = step.pos.y - 1 };
            if (!memory[next_pos.y][next_pos.x]) {
                try queue.append(Step{ .cost = step.cost + 1, .pos = next_pos });
            }
        }
        if (step.pos.y < size) {
            const next_pos = Vec2{ .x = step.pos.x, .y = step.pos.y + 1 };
            if (!memory[next_pos.y][next_pos.x]) {
                try queue.append(Step{ .cost = step.cost + 1, .pos = next_pos });
            }
        }
    }

    return best_path;
}

fn solvePart2(comptime size: usize, bytes: []const Vec2, allocator: std.mem.Allocator) !Vec2 {
    var left: usize = 0;
    var right = bytes.len - 1;

    while (left < right) {
        const mid = left + (right - left) / 2;
        if (try solvePart1(size, bytes[0..mid], allocator) != null) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    return bytes[left - 1];
}

const Step = struct {
    pos: Vec2,
    cost: usize,
};

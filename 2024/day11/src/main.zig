const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var cache = std.hash_map.AutoHashMap(Step, usize).init(allocator);
    const stones = try parseInput(input, allocator);
    var result: usize = 0;
    var result2: usize = 0;
    for (stones.items) |s| {
        result += try count(s, 25, &cache);
        result2 += try count(s, 75, &cache);
    }

    std.debug.print("PART 1: {}\n", .{result});
    std.debug.print("PART 2: {}\n", .{result2});
}

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !std.ArrayList(usize) {
    var result = std.ArrayList(usize).init(allocator);
    var numbers = std.mem.tokenizeScalar(u8, std.mem.trimRight(u8, input, &[_]u8{'\n'}), ' ');

    while (numbers.next()) |number| {
        const value = try std.fmt.parseInt(usize, number, 10);
        try result.append(value);
    }

    return result;
}

const Step = struct {
    stone: usize,
    steps: usize,
};

fn count(stone: usize, steps: usize, cache: *std.AutoHashMap(Step, usize)) std.mem.Allocator.Error!usize {
    if (steps == 0) {
        return 1;
    }
    if (stone == 0) {
        return try countAndCache(1, steps - 1, cache);
    }
    const digits = countDigits(stone);
    if (digits % 2 == 0) {
        const power = std.math.pow(usize, 10, digits / 2);
        const left = stone / power;
        const right = stone % power;

        return try countAndCache(left, steps - 1, cache) +
            try countAndCache(right, steps - 1, cache);
    }

    return try countAndCache(stone * 2024, steps - 1, cache);
}

fn countAndCache(stone: usize, steps: usize, cache: *std.AutoHashMap(Step, usize)) !usize {
    const key = Step{ .stone = stone, .steps = steps };
    if (cache.get(key)) |res| {
        return res;
    }
    const result = try count(stone, steps, cache);
    try cache.put(key, result);
    return result;
}

fn countDigits(number: usize) usize {
    if (number == 0) {
        return 1;
    }
    const power: usize = @intFromFloat(@floor(@log10(@as(f64, @floatFromInt(number)))));
    return power + 1;
}

test "snippet" {
    const input = "125 17";

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var cache = std.hash_map.AutoHashMap(Step, usize).init(allocator);
    const stones = try parseInput(input, allocator);
    var result: usize = 0;
    for (stones.items) |s| {
        result += try count(s, 25, &cache);
    }

    try std.testing.expectEqual(55312, result);
}

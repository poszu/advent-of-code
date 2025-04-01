const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");

    const parsed = try parseInput(input, std.heap.page_allocator);
    defer parsed.map.area.deinit();

    const result = try solve(&parsed.map, parsed.start, std.heap.page_allocator);
    std.debug.print("PART 1: {}\n", .{result.cost});
    std.debug.print("PART 2: {}\n", .{result.count});
}

const Tile = enum {
    end,
    wall,
    path,
};

const Vec2 = struct {
    x: isize,
    y: isize,

    fn add(self: Vec2, other: Vec2) Vec2 {
        return Vec2{
            .x = self.x + other.x,
            .y = self.y + other.y,
        };
    }
};

const Map = struct {
    area: std.ArrayList(std.ArrayList(Tile)),
    end: Vec2,

    fn tile(self: *const Map, pos: Vec2) Tile {
        return self.area.items[@intCast(pos.y)].items[@intCast(pos.x)];
    }
};

const Position = struct {
    loc: Vec2,
    dir: Vec2,
};

const Step = struct {
    cost: usize,
    pos: Position,
    fn compare(_: void, a: Step, b: Step) std.math.Order {
        return std.math.order(a.cost, b.cost);
    }
};

fn solve(map: *const Map, start: Vec2, allocator: std.mem.Allocator) !struct { cost: usize, count: usize } {
    var backtrack = std.AutoHashMap(Position, std.AutoHashMap(Position, void)).init(allocator);
    var lowest_cost = std.AutoHashMap(Position, usize).init(allocator);
    var lowest_next = std.PriorityQueue(Step, void, Step.compare).init(allocator, {});
    try lowest_next.add(Step{ .cost = 0, .pos = Position{ .dir = Vec2{ .x = 1, .y = 0 }, .loc = start } });
    var end_positions = std.AutoHashMap(Position, void).init(allocator);
    var best_cost: usize = std.math.maxInt(usize);

    while (lowest_next.removeOrNull()) |step| {
        if (lowest_cost.get(step.pos)) |cost| {
            if (step.cost > cost) continue;
        }
        if (map.tile(step.pos.loc) == Tile.end) {
            if (step.cost > best_cost) break;
            best_cost = step.cost;
            try end_positions.put(step.pos, {});
        }
        const pos = step.pos;

        for ([_]Step{
            Step{ .cost = step.cost + 1, .pos = Position{ .loc = pos.loc.add(pos.dir), .dir = pos.dir } },
            Step{ .cost = step.cost + 1000, .pos = Position{ .loc = pos.loc, .dir = Vec2{ .x = pos.dir.y, .y = -pos.dir.x } } },
            Step{ .cost = step.cost + 1000, .pos = Position{ .loc = pos.loc, .dir = Vec2{ .x = -pos.dir.y, .y = pos.dir.x } } },
        }) |next_step| {
            if (map.tile(next_step.pos.loc) == Tile.wall) continue;

            var previous = try backtrack.getOrPutValue(next_step.pos, std.AutoHashMap(Position, void).init(allocator));
            const cost = try lowest_cost.getOrPutValue(next_step.pos, next_step.cost);
            if (next_step.cost > cost.value_ptr.*) continue;
            if (next_step.cost < cost.value_ptr.*) {
                // new cheapest way to arrive at next_step.pos
                cost.value_ptr.* = next_step.cost;
                previous.value_ptr.clearRetainingCapacity();
            }

            try previous.value_ptr.put(pos, {});
            try lowest_next.add(next_step);
        }
    }

    // Track back all winning paths
    var unique_points = std.AutoHashMap(Vec2, void).init(allocator);
    var seen = std.AutoHashMap(Position, void).init(allocator);
    var queue = std.ArrayList(Position).init(allocator);
    var end_positions_it = end_positions.keyIterator();
    while (end_positions_it.next()) |pos| {
        try queue.append(pos.*);
    }

    while (queue.pop()) |p| {
        try unique_points.put(p.loc, {});
        if (backtrack.get(p)) |prevs| {
            var prevs_iter = prevs.keyIterator();
            while (prevs_iter.next()) |prev| {
                if (seen.contains(prev.*)) continue;
                try queue.append(prev.*);
                try seen.put(prev.*, {});
            }
        }
    }

    return .{ .cost = best_cost, .count = unique_points.count() };
}

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !struct { map: Map, start: Vec2 } {
    var area = std.ArrayList(std.ArrayList(Tile)).init(allocator);
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var start: ?Vec2 = null;
    var end: ?Vec2 = null;
    var y: u8 = 0;

    while (lines.next()) |line| : (y += 1) {
        var row = std.ArrayList(Tile).init(allocator);

        for (0.., line) |x, c| {
            const tile = switch (c) {
                '#' => Tile.wall,
                'E' => blk: {
                    end = Vec2{ .x = @intCast(x), .y = y };
                    break :blk Tile.end;
                },
                '.' => Tile.path,
                'S' => blk: {
                    start = Vec2{ .x = @intCast(x), .y = y };
                    break :blk Tile.path;
                },
                else => unreachable,
            };
            try row.append(tile);
        }
        try area.append(row);
    }

    return .{ .map = Map{ .end = end.?, .area = area }, .start = start.? };
}

test "part 1 and 2" {
    const input =
        \\###############
        \\#.......#....E#
        \\#.#.###.#.###.#
        \\#.....#.#...#.#
        \\#.###.#####.#.#
        \\#.#.#.......#.#
        \\#.#.#####.###.#
        \\#...........#.#
        \\###.#.#####.#.#
        \\#...#.....#.#.#
        \\#.#.#.###.#.#.#
        \\#.....#...#.#.#
        \\#.###.#.#.#.#.#
        \\#S..#.....#...#
        \\###############
    ;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const parsed = try parseInput(input, allocator);

    try std.testing.expectEqual(Vec2{ .x = 1, .y = 13 }, parsed.start);

    const result = try solve(&parsed.map, parsed.start, allocator);

    try std.testing.expectEqual(7036, result.cost);
    try std.testing.expectEqual(45, result.count);
}

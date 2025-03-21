const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const map = try parse_input(input, allocator);

    var scores: usize = 0;
    var ratings: usize = 0;
    for (map.trailheads.items) |head| {
        const result = try map.score_trailhead(head, allocator);
        scores += result.score;
        ratings += result.rating;
    }

    std.debug.print("PART 1: {}\n", .{scores});
    std.debug.print("PART 2: {}\n", .{ratings});
}

const Coords = struct {
    x: usize,
    y: usize,
};

const Position = struct {
    coords: Coords,
    height: usize,
};

const Path = struct {
    current: Position,
    visited: std.AutoHashMap(Coords, void),

    fn new(coord: Coords, allocator: std.mem.Allocator) !Path {
        var visited = std.AutoHashMap(Coords, void).init(allocator);
        try visited.put(coord, {});
        return Path{
            .current = Position{ .coords = coord, .height = 0 },
            .visited = visited,
        };
    }

    fn clone(self: *const Path) !Path {
        return Path{
            .current = self.current,
            .visited = try self.visited.clone(),
        };
    }

    fn move(self: *Path, coords: Coords, value: usize) !void {
        try self.visited.put(coords, {});
        self.current = Position{ .coords = coords, .height = value };
    }

    fn height(self: *const Path) usize {
        return self.position().height;
    }

    fn position(self: *const Path) Position {
        return self.current;
    }
};

const Map = struct {
    width: usize,
    height: usize,
    grid: std.ArrayList(std.ArrayList(u8)),
    trailheads: std.ArrayList(Coords),

    fn deinit(self: *const Map) void {
        for (self.grid.items) |r| r.deinit();
        self.grid.deinit();
        self.trailheads.deinit();
    }

    fn terrain_height(self: *const Map, coord: Coords) usize {
        return self.grid.items[coord.y].items[coord.x];
    }

    fn try_go(self: *const Map, new_pos: Coords, path: *const Path, queue: *std.ArrayList(Path)) !bool {
        if (path.visited.contains(new_pos)) {
            return false;
        }
        const new_pos_height = self.terrain_height(new_pos);
        if (new_pos_height == path.height() + 1) {
            if (new_pos_height == 9) {
                return true;
            }
            var new_path = try path.clone();
            try new_path.move(new_pos, new_pos_height);
            try queue.append(new_path);
        }
        return false;
    }

    fn score_trailhead(self: *const Map, coord: Coords, allocator: std.mem.Allocator) !struct { score: usize, rating: usize } {
        var distinct_paths: usize = 0;
        var tops = std.AutoHashMap(Coords, void).init(allocator);
        var queue = std.ArrayList(Path).init(allocator);
        try queue.append(try Path.new(coord, allocator));
        while (queue.pop()) |path| {
            const pos = path.position().coords;
            // try go down
            if (pos.y < self.height - 1) {
                const new_pos = Coords{ .x = pos.x, .y = pos.y + 1 };
                if (try self.try_go(new_pos, &path, &queue)) {
                    try tops.put(new_pos, {});
                    distinct_paths += 1;
                }
            }
            // try go up
            if (pos.y > 0) {
                const new_pos = Coords{ .x = pos.x, .y = pos.y - 1 };
                if (try self.try_go(new_pos, &path, &queue)) {
                    try tops.put(new_pos, {});
                    distinct_paths += 1;
                }
            }
            // try go left
            if (pos.x > 0) {
                const new_pos = Coords{ .x = pos.x - 1, .y = pos.y };
                if (try self.try_go(new_pos, &path, &queue)) {
                    try tops.put(new_pos, {});
                    distinct_paths += 1;
                }
            }
            // try go right
            if (pos.x < self.width - 1) {
                const new_pos = Coords{ .x = pos.x + 1, .y = pos.y };
                if (try self.try_go(new_pos, &path, &queue)) {
                    try tops.put(new_pos, {});
                    distinct_paths += 1;
                }
            }
        }

        return .{ .score = tops.count(), .rating = distinct_paths };
    }
};

fn parse_input(input: []const u8, allocator: std.mem.Allocator) !Map {
    var map = std.ArrayList(std.ArrayList(u8)).init(allocator);
    var trailheads = std.ArrayList(Coords).init(allocator);

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var y: usize = 0;

    while (lines.next()) |line| {
        var row = std.ArrayList(u8).init(allocator);
        for (0.., line) |x, c| {
            const height = try std.fmt.charToDigit(c, 10);
            try row.append(height);
            if (height == 0) {
                try trailheads.append(Coords{ .x = x, .y = y });
            }
        }
        try map.append(row);
        y += 1;
    }

    return Map{
        .width = map.items[0].items.len,
        .height = y,
        .grid = map,
        .trailheads = trailheads,
    };
}

test "part1" {
    const input =
        \\89010123
        \\78121874
        \\87430965
        \\96549874
        \\45678903
        \\32019012
        \\01329801
        \\10456732
    ;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const map = try parse_input(input, allocator);

    var scores: usize = 0;
    var ratings: usize = 0;
    for (map.trailheads.items) |head| {
        const result = try map.score_trailhead(head, allocator);
        scores += result.score;
        ratings += result.rating;
    }
    try std.testing.expectEqual(36, scores);
    try std.testing.expectEqual(81, ratings);
}

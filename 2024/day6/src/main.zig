const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    const parsed = try parseMap(input);
    const positions = try simulatePath(parsed[0], parsed[1]);
    try stdout.print("[PART 1] total distinct positions: {}\n", .{positions.Steps.items.len});

    const loops = try findInfiniteLoops(parsed[0].items, positions.Steps.items);
    try stdout.print("[PART 2] total infinite loops: {}\n", .{loops});
}

const Point = struct {
    x: usize,
    y: usize,

    pub fn eql(self: Point, other: Point) bool {
        return self.x == other.x and self.y == other.y;
    }

    pub fn hash(self: Point) u64 {
        var h = std.hash.Wyhash.init(0);
        h.update(&std.mem.toBytes(self.x));
        h.update(&std.mem.toBytes(self.y));
        return h.final();
    }
};

const Spot = enum(u1) { Free, Occupied };

const Position = struct {
    coords: Point,
    orientation: u8,

    pub fn eql(self: Position, other: Position) bool {
        return self.coords.eql(other.coords) and self.orientation == other.orientation;
    }

    pub fn hash(self: Position) u64 {
        var h = std.hash.Wyhash.init(0);
        h.update(&std.mem.toBytes(self.coords.hash()));
        h.update(&std.mem.toBytes(self.orientation));
        return h.final();
    }
};

fn parseMap(input: []const u8) !struct { std.ArrayList(std.ArrayList(Spot)), Position } {
    var lines = std.mem.splitScalar(u8, input, '\n');
    var y: usize = 0;
    var grid = std.ArrayList(std.ArrayList(Spot)).init(std.heap.page_allocator);
    var pos = Position{ .coords = Point{ .x = 0, .y = 0 }, .orientation = 0 };
    while (lines.next()) |line| {
        if (line.len == 0) {
            break;
        }
        var row = std.ArrayList(Spot).init(std.heap.page_allocator);
        for (0.., line) |x, v| {
            switch (v) {
                '.' => {
                    try row.append(Spot.Free);
                },
                '#' => {
                    try row.append(Spot.Occupied);
                },
                '<', '>', '^', 'v' => {
                    try row.append(Spot.Free);
                    pos = Position{ .coords = Point{ .x = x, .y = y }, .orientation = v };
                },
                else => unreachable,
            }
        }
        try grid.append(row);
        y += 1;
    }
    return .{ grid, pos };
}

const Path = union(enum) {
    Steps: std.ArrayList(Position),
    Infinite: void,
};

fn simulatePath(grid: std.ArrayList(std.ArrayList(Spot)), p: Position) !Path {
    const width = grid.items[0].items.len;
    const height = grid.items.len;
    var pos = p;
    var steps = std.ArrayList(Position).init(std.heap.page_allocator);
    var visited = std.AutoArrayHashMap(Point, void).init(std.heap.page_allocator);
    var visited_with_orient = std.AutoArrayHashMap(Position, void).init(std.heap.page_allocator);

    while (true) {
        const entry = try visited.getOrPutValue(pos.coords, {});
        if (!entry.found_existing) {
            try steps.append(pos);
        }
        const entry_orient = try visited_with_orient.getOrPutValue(pos, {});
        if (entry_orient.found_existing) {
            // it's infinite loop
            return Path{ .Infinite = {} };
        }
        const coords = pos.coords;
        switch (pos.orientation) {
            '^' => {
                if (coords.y == 0) {
                    break;
                }
                if (grid.items[coords.y - 1].items[coords.x] == Spot.Occupied) {
                    pos.orientation = '>';
                } else {
                    pos.coords.y -= 1;
                }
            },
            '>' => {
                if (coords.x == width - 1) {
                    break;
                }
                if (grid.items[coords.y].items[coords.x + 1] == Spot.Occupied) {
                    pos.orientation = 'v';
                } else {
                    pos.coords.x += 1;
                }
            },
            'v' => {
                if (coords.y == height - 1) {
                    break;
                }
                if (grid.items[coords.y + 1].items[coords.x] == Spot.Occupied) {
                    pos.orientation = '<';
                } else {
                    pos.coords.y += 1;
                }
            },
            '<' => {
                if (coords.x == 0) {
                    break;
                }
                if (grid.items[coords.y].items[coords.x - 1] == Spot.Occupied) {
                    pos.orientation = '^';
                } else {
                    pos.coords.x -= 1;
                }
            },
            else => unreachable,
        }
    }
    return Path{ .Steps = steps };
}

fn findInfiniteLoops(grid: []std.ArrayList(Spot), path: []const Position) !usize {
    var loops: usize = 0;
    var grid_copy = std.ArrayList(std.ArrayList(Spot)).init(std.heap.page_allocator);
    defer grid_copy.deinit();
    for (grid) |_| {
        const new_row = std.ArrayList(Spot).init(std.heap.page_allocator);
        try grid_copy.append(new_row);
    }

    for (path[1..]) |pos| {
        // make a copy of the grid
        for (0.., grid) |i, row| {
            grid_copy.items[i].clearRetainingCapacity();
            try grid_copy.items[i].appendSlice(row.items);
        }
        // place an obstacle
        grid_copy.items[pos.coords.y].items[pos.coords.x] = Spot.Occupied;
        const simulatedPath = try simulatePath(grid_copy, path[0]);
        switch (simulatedPath) {
            .Infinite => {
                loops += 1;
            },
            .Steps => {},
        }
    }

    return loops;
}

test "test vector" {
    const input =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;
    const parsed = try parseMap(input);
    try std.testing.expectEqual(Position{ .coords = Point{ .x = 4, .y = 6 }, .orientation = '^' }, parsed[1]);
    const result = try simulatePath(parsed[0], parsed[1]);
    try std.testing.expectEqual(41, result.Steps.items.len);

    const loops = try findInfiniteLoops(parsed[0].items, result.Steps.items);
    try std.testing.expectEqual(6, loops);
}

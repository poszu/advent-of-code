const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = @embedFile("input.txt");
    var parsed = try parseInput(input, allocator);
    var map_part2 = try evolveMap(&parsed.map, allocator);

    runMoves(&parsed.map, parsed.moves);

    const result = sumBoxCoordinates(&parsed.map);
    std.debug.print("PART 1: {}\n", .{result});

    for (parsed.moves) |dir| {
        if (dir == '\n') {
            continue;
        }
        try map_part2.move2(dir, allocator);
    }

    const result2 = sumDoubleBoxCoordinates(&map_part2);
    std.debug.print("PART 2: {}\n", .{result2});
}

const Position = struct {
    x: usize,
    y: usize,

    fn up(self: *const Position) Position {
        return Position{ .x = self.x, .y = self.y - 1 };
    }
    fn down(self: *const Position) Position {
        return Position{ .x = self.x, .y = self.y + 1 };
    }
    fn left(self: *const Position) Position {
        return Position{ .x = self.x - 1, .y = self.y };
    }
    fn right(self: *const Position) Position {
        return Position{ .x = self.x + 1, .y = self.y };
    }
};

const Map = struct {
    width: usize,
    area: std.ArrayList(std.ArrayList(u8)),
    robot: Position,

    pub fn format(
        self: *const Map,
        comptime _: []const u8,
        _: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        for (self.area.items) |row| {
            for (row.items) |c| {
                try writer.print("{c}", .{c});
            }
            try writer.print("\n", .{});
        }
    }

    fn cell(self: *const Map, pos: Position) u8 {
        return self.area.items[pos.y].items[pos.x];
    }

    fn doubleCell(self: *const Map, pos: Position) [2]u8 {
        return [2]u8{
            self.cell(pos),
            self.cell(Position{ .x = pos.x + 1, .y = pos.y }),
        };
    }

    fn set_cell(self: *Map, pos: Position, value: u8) void {
        self.area.items[pos.y].items[pos.x] = value;
    }

    fn setDoubleCell(self: *Map, pos: Position, value: []const u8) void {
        self.area.items[pos.y].items[pos.x] = value[0];
        self.area.items[pos.y].items[pos.x + 1] = value[1];
    }

    fn move(self: *Map, dir: u8) void {
        switch (dir) {
            '>' => {
                const next_pos = Position{ .x = self.robot.x + 1, .y = self.robot.y };
                const next_cell = self.cell(next_pos);
                switch (next_cell) {
                    '#' => {},
                    'O' => {
                        var pos = next_pos.right();
                        while (pos.x < self.width - 1) : (pos = pos.right()) {
                            switch (self.cell(pos)) {
                                '.' => {
                                    self.set_cell(pos, 'O');
                                    self.moveRobot(next_pos);
                                    break;
                                },
                                '#' => {
                                    break;
                                },
                                else => {},
                            }
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            '<' => {
                const next_pos = Position{ .x = self.robot.x - 1, .y = self.robot.y };
                const next_cell = self.cell(next_pos);
                switch (next_cell) {
                    '#' => {},
                    'O' => {
                        var pos = next_pos.left();
                        while (pos.x > 0) : (pos = pos.left()) {
                            switch (self.cell(pos)) {
                                '.' => {
                                    self.set_cell(pos, 'O');
                                    self.moveRobot(next_pos);
                                    break;
                                },
                                '#' => {
                                    break;
                                },
                                else => {},
                            }
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            '^' => {
                const next_pos = Position{ .x = self.robot.x, .y = self.robot.y - 1 };
                const next_cell = self.cell(next_pos);
                switch (next_cell) {
                    '#' => {},
                    'O' => {
                        var pos = next_pos.up();
                        while (pos.y > 0) : (pos = pos.up()) {
                            switch (self.cell(pos)) {
                                '.' => {
                                    self.set_cell(pos, 'O');
                                    self.moveRobot(next_pos);
                                    break;
                                },
                                '#' => {
                                    break;
                                },
                                else => {},
                            }
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            'v' => {
                const next_pos = Position{ .x = self.robot.x, .y = self.robot.y + 1 };
                const next_cell = self.cell(next_pos);
                switch (next_cell) {
                    '#' => {},
                    'O' => {
                        var pos = next_pos.down();
                        while (pos.y < self.area.items.len) : (pos = pos.down()) {
                            switch (self.cell(pos)) {
                                '.' => {
                                    self.set_cell(pos, 'O');
                                    self.moveRobot(next_pos);
                                    break;
                                },
                                '#' => {
                                    break;
                                },
                                else => {},
                            }
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            else => {
                std.debug.panic("invalid direction {}", .{dir});
            },
        }
    }

    fn moveRobot(self: *Map, new_pos: Position) void {
        self.set_cell(self.robot, '.');
        self.set_cell(new_pos, '@');
        self.robot = new_pos;
    }

    fn move2(self: *Map, dir: u8, allocator: std.mem.Allocator) !void {
        switch (dir) {
            '>' => {
                const next_pos = self.robot.right();
                switch (self.cell(next_pos)) {
                    '#' => {},
                    '[' => {
                        var pos = next_pos.right();
                        while (pos.x < self.width) : (pos = pos.right()) {
                            switch (self.cell(pos)) {
                                '.' => {
                                    while (pos.x > next_pos.x) : (pos = pos.left().left()) {
                                        self.setDoubleCell(pos.left(), "[]");
                                    }
                                    self.moveRobot(next_pos);
                                    break;
                                },
                                '#' => {
                                    break;
                                },
                                else => {},
                            }
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            '<' => {
                const next_pos = self.robot.left();
                switch (self.cell(next_pos)) {
                    '#' => {},
                    ']' => {
                        var pos = next_pos.left();
                        while (pos.x > 0) : (pos = pos.left()) {
                            switch (self.cell(pos)) {
                                '.' => {
                                    while (pos.x < next_pos.x) : (pos = pos.right().right()) {
                                        self.setDoubleCell(pos, "[]");
                                    }
                                    self.moveRobot(next_pos);
                                    break;
                                },
                                '#' => {
                                    break;
                                },
                                else => {},
                            }
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            'v' => {
                const next_pos = self.robot.down();
                switch (self.cell(next_pos)) {
                    '#' => {},
                    '[' => {
                        if (try self.moveDoubleBoxDown(next_pos, allocator)) {
                            self.moveRobot(next_pos);
                        }
                    },
                    ']' => {
                        if (try self.moveDoubleBoxDown(next_pos.left(), allocator)) {
                            self.moveRobot(next_pos);
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            '^' => {
                const next_pos = self.robot.up();
                switch (self.cell(next_pos)) {
                    '#' => {},
                    '[' => {
                        if (try self.moveDoubleBoxUp(next_pos, allocator)) {
                            self.moveRobot(next_pos);
                        }
                    },
                    ']' => {
                        if (try self.moveDoubleBoxUp(next_pos.left(), allocator)) {
                            self.moveRobot(next_pos);
                        }
                    },
                    else => {
                        self.moveRobot(next_pos);
                    },
                }
            },
            else => {},
        }
    }

    fn moveDoubleBoxUp(self: *Map, pos: Position, allocator: std.mem.Allocator) !bool {
        var stack = std.ArrayList(Position).init(allocator);
        defer stack.deinit();
        var moved = std.AutoHashMap(Position, void).init(allocator);
        defer moved.deinit();
        if (try self.tryMoveDoubleBoxUp(pos, &stack)) {
            for (stack.items) |box| {
                if (moved.contains(box)) {
                    continue;
                }
                try moved.put(box, {});
                self.setDoubleCell(box.up(), "[]");
                self.setDoubleCell(box, "..");
            }
            return true;
        }
        return false;
    }

    fn tryMoveDoubleBoxUp(self: *Map, pos: Position, stack: *std.ArrayList(Position)) !bool {
        const next_pos = pos.up();
        const next_cell = self.doubleCell(next_pos);
        if (std.mem.eql(u8, &next_cell, "..")) {
            try stack.append(pos);
            return true;
        }
        if (next_cell[0] == '#' or next_cell[1] == '#') {
            return false;
        }
        if (std.mem.eql(u8, &next_cell, "[]")) {
            if (!try self.tryMoveDoubleBoxUp(next_pos, stack)) {
                return false;
            }
        }
        if (next_cell[0] == ']') {
            if (!try self.tryMoveDoubleBoxUp(next_pos.left(), stack)) {
                return false;
            }
        }
        if (next_cell[1] == '[') {
            if (!try self.tryMoveDoubleBoxUp(next_pos.right(), stack)) {
                return false;
            }
        }
        try stack.append(pos);
        return true;
    }

    fn moveDoubleBoxDown(self: *Map, pos: Position, allocator: std.mem.Allocator) !bool {
        var stack = std.ArrayList(Position).init(allocator);
        defer stack.deinit();
        var moved = std.AutoHashMap(Position, void).init(allocator);
        defer moved.deinit();
        if (try self.tryMoveDoubleBoxDown(pos, &stack)) {
            for (stack.items) |box| {
                if (moved.contains(box)) {
                    continue;
                }
                try moved.put(box, {});
                self.setDoubleCell(box.down(), "[]");
                self.setDoubleCell(box, "..");
            }
            return true;
        }
        return false;
    }

    fn tryMoveDoubleBoxDown(self: *Map, pos: Position, stack: *std.ArrayList(Position)) !bool {
        const next_pos = pos.down();
        const next_cell = self.doubleCell(next_pos);
        if (std.mem.eql(u8, &next_cell, "..")) {
            try stack.append(pos);
            return true;
        }
        if (next_cell[0] == '#' or next_cell[1] == '#') {
            return false;
        }
        if (std.mem.eql(u8, &next_cell, "[]")) {
            if (!try self.tryMoveDoubleBoxDown(next_pos, stack)) {
                return false;
            }
        }
        if (next_cell[0] == ']') {
            if (!try self.tryMoveDoubleBoxDown(next_pos.left(), stack)) {
                return false;
            }
        }
        if (next_cell[1] == '[') {
            if (!try self.tryMoveDoubleBoxDown(next_pos.right(), stack)) {
                return false;
            }
        }
        try stack.append(pos);
        return true;
    }
};

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !struct { map: Map, moves: []const u8 } {
    const area_end = std.mem.indexOf(u8, input, "\n\n").?;
    const width = std.mem.indexOfScalar(u8, input, '\n').?;

    var area = std.ArrayList(std.ArrayList(u8)).init(allocator);
    var lines = std.mem.tokenizeScalar(u8, input[0..area_end], '\n');
    var robot: ?Position = null;

    var y: usize = 0;
    while (lines.next()) |line| : (y += 1) {
        for (0.., line) |x, cell| {
            if (cell == '@') {
                robot = Position{ .x = x, .y = y };
            }
        }

        var row = std.ArrayList(u8).init(allocator);
        try row.appendSlice(line);
        try area.append(row);
    }

    return .{
        .map = Map{
            .width = width,
            .area = area,
            .robot = robot.?,
        },
        .moves = input[area_end + 2 ..],
    };
}

fn runMoves(map: *Map, moves: []const u8) void {
    for (moves) |dir| {
        if (dir == '\n') {
            continue;
        }
        map.move(dir);
    }
}

fn sumBoxCoordinates(map: *const Map) usize {
    var result: usize = 0;
    for (0.., map.area.items) |y, row| {
        for (0.., row.items) |x, cell| {
            if (cell == 'O') {
                result += 100 * y + x;
            }
        }
    }

    return result;
}

fn sumDoubleBoxCoordinates(map: *const Map) usize {
    var result: usize = 0;
    for (0.., map.area.items) |y, row| {
        for (0.., row.items) |x, cell| {
            if (cell == '[') {
                result += 100 * y + x;
            }
        }
    }

    return result;
}

// evolve map for part 2
fn evolveMap(map: *const Map, allocator: std.mem.Allocator) !Map {
    var new_area = try std.ArrayList(std.ArrayList(u8)).initCapacity(allocator, map.area.items.len);
    for (map.area.items) |row| {
        var new_row = try std.ArrayList(u8).initCapacity(allocator, map.width * 2);
        for (row.items) |cell| {
            switch (cell) {
                'O' => {
                    try new_row.appendSlice("[]");
                },
                '@' => {
                    try new_row.appendSlice("@.");
                },
                else => {
                    try new_row.appendNTimes(cell, 2);
                },
            }
        }
        try new_area.append(new_row);
    }

    return Map{
        .area = new_area,
        .width = map.width * 2,
        .robot = Position{ .x = map.robot.x * 2, .y = map.robot.y },
    };
}

const big_example =
    \\##########
    \\#..O..O.O#
    \\#......O.#
    \\#.OO..O.O#
    \\#..O@..O.#
    \\#O#..O...#
    \\#O..O..O.#
    \\#.OO.O.OO#
    \\#....O...#
    \\##########
    \\
    \\<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    \\vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    \\><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    \\<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    \\^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    \\^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    \\>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    \\<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    \\^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    \\v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
;

test "part 1" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var parsed = try parseInput(big_example, allocator);
    try std.testing.expectEqual(10, parsed.map.width);
    try std.testing.expectEqual(10, parsed.map.area.items.len);
    try std.testing.expectEqual(Position{ .x = 4, .y = 4 }, parsed.map.robot);

    runMoves(&parsed.map, parsed.moves);

    const result = sumBoxCoordinates(&parsed.map);
    try std.testing.expectEqual(10092, result);
}

test "part 2" {
    const input =
        \\#######
        \\#...#.#
        \\#.....#
        \\#..OO@#
        \\#..O..#
        \\#.....#
        \\#######
        \\
        \\<vv<<^^<<^^
    ;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var parsed = try parseInput(input, allocator);

    var map = try evolveMap(&parsed.map, allocator);

    std.debug.print("{}", .{map});
    for (parsed.moves) |dir| {
        if (dir == '\n') {
            continue;
        }
        try map.move2(dir, allocator);
    }
    std.debug.print("{}", .{map});
}
test "part 2 big" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var parsed = try parseInput(big_example, allocator);

    var map = try evolveMap(&parsed.map, allocator);

    std.debug.print("{}", .{map});
    for (parsed.moves) |dir| {
        if (dir == '\n') {
            continue;
        }
        try map.move2(dir, allocator);
    }
    std.debug.print("{}", .{map});

    const result = sumDoubleBoxCoordinates(&map);
    try std.testing.expectEqual(9021, result);
}

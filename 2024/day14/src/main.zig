const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");

    const robots = try parseInput(input, std.heap.page_allocator);
    defer robots.deinit();

    var quadrants = [4]usize{ 0, 0, 0, 0 };

    for (robots.items) |*robot| {
        for (0..100) |_| {
            robot.move(.{ .x = 101, .y = 103 });
        }
        if (whichQuadrant(robot.*, .{ .x = 101, .y = 103 })) |q| {
            quadrants[q] += 1;
        }
    }

    var total: usize = 1;
    for (quadrants) |q| {
        total *= q;
    }

    std.debug.print("PART 1: {}\n", .{total});

    const robots2 = try parseInput(input, std.heap.page_allocator);
    defer robots2.deinit();

    var min_concetration_factor: usize = std.math.maxInt(usize);
    for (1..10_000) |steps| {
        var quads = [4]usize{ 0, 0, 0, 0 };
        for (robots2.items) |*robot| {
            robot.move(.{ .x = 101, .y = 103 });
            if (whichQuadrant(robot.*, .{ .x = 101, .y = 103 })) |q| {
                quads[q] += 1;
            }
        }

        // If robots are concentrated in a single quadrant, then the product of all
        // should be minimal (it's the largest if robots are evenly spread).
        // NOTE: It's a guess that tree picture is located in one of the quadrants.
        const concetration_factor = quads[0] * quads[1] * quads[2] * quads[3];
        if (concetration_factor < min_concetration_factor) {
            min_concetration_factor = concetration_factor;
            std.debug.print("Possible christmas  tree location after {} steps\n", .{steps});
            draw(robots2.items);
        }
    }
}

const Robot = struct {
    x: isize,
    y: isize,

    vx: isize,
    vy: isize,

    fn move(self: *Robot, bounds: struct { x: isize, y: isize }) void {
        self.x = @mod(self.x + self.vx, bounds.x);
        self.y = @mod(self.y + self.vy, bounds.y);
    }
};

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Robot) {
    var robots = std.ArrayList(Robot).init(allocator);
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        const robot = parseRobot(line);
        try robots.append(robot);
    }

    return robots;
}

fn parseRobot(line: []const u8) Robot {
    const x_end = std.mem.indexOfScalar(u8, line, ',').?;
    const x = std.fmt.parseInt(isize, line[2..x_end], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    const y_end = std.mem.indexOfScalar(u8, line, ' ').?;
    const y = std.fmt.parseInt(isize, line[x_end + 1 .. y_end], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    const vx_end = std.mem.indexOfScalarPos(u8, line, y_end, ',').?;
    const vx = std.fmt.parseInt(isize, line[y_end + 3 .. vx_end], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    const vy = std.fmt.parseInt(isize, line[vx_end + 1 ..], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    return Robot{
        .x = x,
        .y = y,
        .vx = vx,
        .vy = vy,
    };
}

fn draw(robots: []const Robot) void {
    const width: usize = 101;
    const height: usize = 103;

    // Create a grid filled with spaces
    var grid: [height][width]u8 = undefined;
    for (0..height) |y| {
        for (0..width) |x| {
            grid[y][x] = ' ';
        }
    }

    // Mark each robot's position with '#'
    for (robots) |robot| {
        const x: usize = @intCast(@mod(robot.x, width));
        const y: usize = @intCast(@mod(robot.y, height));
        grid[y][x] = '#';
    }

    // Print the grid
    for (0..height) |y| {
        for (0..width) |x| {
            std.debug.print("{c}", .{grid[y][x]});
        }
        std.debug.print("\n", .{});
    }
}

test "parsing robot" {
    const robot = parseRobot("p=0,4 v=3,-3");
    try std.testing.expectEqual(Robot{ .x = 0, .y = 4, .vx = 3, .vy = -3 }, robot);
}

test "part1" {
    const input =
        \\p=0,4 v=3,-3
        \\p=6,3 v=-1,-3
        \\p=10,3 v=-1,2
        \\p=2,0 v=2,-1
        \\p=0,0 v=1,3
        \\p=3,0 v=-2,-2
        \\p=7,6 v=-1,-3
        \\p=3,0 v=-1,-2
        \\p=9,3 v=2,3
        \\p=7,3 v=-1,2
        \\p=2,4 v=2,-3
        \\p=9,5 v=-3,-3
    ;

    const robots = try parseInput(input, std.testing.allocator);
    defer robots.deinit();

    var quadrants = [4]usize{ 0, 0, 0, 0 };

    for (robots.items) |*robot| {
        for (0..100) |_| {
            robot.move(.{ .x = 11, .y = 7 });
        }
        if (whichQuadrant(robot.*, .{ .x = 11, .y = 7 })) |q| {
            quadrants[q] += 1;
        }
    }

    var total: usize = 1;
    for (quadrants) |q| {
        total *= q;
    }

    try std.testing.expectEqual(12, total);
}

fn whichQuadrant(robot: Robot, bounds: struct { x: isize, y: isize }) ?usize {
    const x_half = @divFloor(bounds.x, 2);
    const y_half = @divFloor(bounds.y, 2);

    if (robot.x == x_half or robot.y == y_half) {
        return null;
    }
    if (robot.x < x_half) {
        if (robot.y < y_half) {
            return 0;
        }
        return 1;
    }
    if (robot.y < y_half) {
        return 2;
    }
    return 3;
}

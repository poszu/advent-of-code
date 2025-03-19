const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    var map = try parseMap(input);
    defer map.map.deinit();
    var antennas = map.map.iterator();

    var antinodes = std.AutoHashMap(Point, void).init(std.heap.page_allocator);
    while (antennas.next()) |entry| {
        for (entry.value_ptr.items) |p1| {
            for (entry.value_ptr.items) |p2| {
                if (p1.x == p2.x and p1.y == p2.y) {
                    continue;
                }
                const x_dist = p2.x - p1.x;
                const y_dist = p2.y - p1.y;

                var n = Point{ .x = p2.x + x_dist, .y = p2.y + y_dist };
                while (n.x >= 0 and n.x < map.width and n.y >= 0 and n.y < map.height) {
                    try antinodes.put(n, {});
                    n.x += x_dist;
                    n.y += y_dist;
                }

                n = Point{ .x = p1.x - x_dist, .y = p1.y - y_dist };
                while (n.x >= 0 and n.x < map.width and n.y >= 0 and n.y < map.height) {
                    try antinodes.put(n, {});
                    n.x -= x_dist;
                    n.y -= y_dist;
                }
            }
        }
    }

    try stdout.print("number of antinodes: {}", .{antinodes.count()});
}

const Point = struct {
    x: i32,
    y: i32,
};

fn parseMap(input: []const u8) !struct { map: std.AutoHashMap(u8, std.ArrayList(Point)), width: i32, height: i32 } {
    var map = std.AutoHashMap(u8, std.ArrayList(Point)).init(std.heap.page_allocator);
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    const width = lines.peek().?.len;
    var y: i32 = 0;
    while (lines.next()) |row| {
        for (0.., row) |x, c| {
            if (c != '.') {
                var entry = try map.getOrPutValue(c, std.ArrayList(Point).init(std.heap.page_allocator));
                try entry.value_ptr.append(Point{ .x = @intCast(x), .y = y });
            }
        }
        y += 1;
    }
    return .{ .map = map, .width = @intCast(width), .height = y };
}

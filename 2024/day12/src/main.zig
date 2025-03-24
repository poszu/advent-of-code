const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    const garden = try parseInput(input, allocator);
    const regions = try findAllRegions(&garden, allocator);
    const price = findPrice(&garden, regions.items);

    std.debug.print("PART 1: {}\n", .{price});

    const price2 = findPrice2(&garden, regions.items);
    std.debug.print("PART 2: {}\n", .{price2});
}

const Garden = struct {
    width: usize,
    height: usize,
    area: std.ArrayList([]const u8),

    fn plot(self: *const Garden, coords: Coordinates) u8 {
        if (coords.y < 0 or coords.y >= self.height) {
            return 0;
        }
        if (coords.x < 0 or coords.x >= self.width) {
            return 0;
        }
        return self.area.items[@intCast(coords.y)][@intCast(coords.x)];
    }

    fn nthCoords(self: *const Garden, n: usize) Coordinates {
        std.debug.assert(n < self.width * self.height);
        return Coordinates{
            .x = @intCast(n % self.width),
            .y = @intCast(n / self.width),
        };
    }

    fn size(self: *const Garden) usize {
        return self.width * self.height;
    }
};

const Coordinates = struct {
    x: isize,
    y: isize,

    fn left(self: Coordinates) Coordinates {
        return Coordinates{
            .x = self.x - 1,
            .y = self.y,
        };
    }
    fn right(self: Coordinates) Coordinates {
        return Coordinates{
            .x = self.x + 1,
            .y = self.y,
        };
    }
    fn down(self: Coordinates) Coordinates {
        return Coordinates{
            .x = self.x,
            .y = self.y + 1,
        };
    }
    fn up(self: Coordinates) Coordinates {
        return Coordinates{
            .x = self.x,
            .y = self.y - 1,
        };
    }
};

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !Garden {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    var height: usize = 0;

    var area = std.ArrayList([]const u8).init(allocator);
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |l| {
        try area.append(l);
        height += 1;
    }

    return Garden{
        .width = width,
        .height = height,
        .area = area,
    };
}

fn findRegion(garden: *const Garden, start: Coordinates, allocator: std.mem.Allocator) ![]Coordinates {
    var region = std.AutoArrayHashMap(Coordinates, void).init(allocator);

    const plant = garden.plot(start);
    var queue = std.ArrayList(Coordinates).init(allocator);
    try queue.append(start);

    while (queue.pop()) |pos| {
        const plot = garden.plot(pos);
        if (plot != plant) {
            continue;
        }
        try region.put(pos, {});
        // try go right
        if (pos.x < garden.width - 1) {
            const new_pos = pos.right();
            if (!region.contains(new_pos)) {
                try queue.append(new_pos);
            }
        }
        // try go left
        if (pos.x > 0) {
            const new_pos = pos.left();
            if (!region.contains(new_pos)) {
                try queue.append(new_pos);
            }
        }
        // try go down
        if (pos.y < garden.height - 1) {
            const new_pos = pos.down();
            if (!region.contains(new_pos)) {
                try queue.append(new_pos);
            }
        }
        // try go up
        if (pos.y > 0) {
            const new_pos = pos.up();
            if (!region.contains(new_pos)) {
                try queue.append(new_pos);
            }
        }
    }

    return region.keys();
}

fn findAllRegions(garden: *const Garden, allocator: std.mem.Allocator) !std.ArrayList([]Coordinates) {
    var regions = std.ArrayList([]Coordinates).init(allocator);
    var visited = std.AutoHashMap(Coordinates, void).init(allocator);
    for (0..garden.size()) |n| {
        const start = garden.nthCoords(n);

        if (visited.contains(start)) {
            continue;
        }
        const region = try findRegion(garden, start, allocator);
        for (region) |coord| {
            try visited.put(coord, {});
        }
        try regions.append(region);
    }
    return regions;
}

fn findPerimeter(garden: *const Garden, region: []Coordinates) usize {
    var perimiter: usize = 0;
    const plot_type = garden.plot(region[0]);
    for (region) |pos| {
        if (garden.plot(pos.right()) != plot_type) {
            perimiter += 1;
        }
        if (garden.plot(pos.left()) != plot_type) {
            perimiter += 1;
        }
        if (garden.plot(pos.down()) != plot_type) {
            perimiter += 1;
        }
        if (garden.plot(pos.up()) != plot_type) {
            perimiter += 1;
        }
    }

    return perimiter;
}

// count sides by counting corners (it's the same number).
fn countSides(garden: *const Garden, region: []Coordinates) usize {
    var corners: usize = 0;
    const plant = garden.plot(region[0]);
    for (region) |pos| {
        const left = pos.left();
        const right = pos.right();
        const above = pos.up();
        const below = pos.down();

        // check if top-left outside
        if (garden.plot(left) != plant and garden.plot(above) != plant) {
            corners += 1;
        }
        // check if top-left inside
        {
            const diagonal = pos.up().left();
            if (garden.plot(left) == plant and garden.plot(above) == plant and garden.plot(diagonal) != plant) {
                corners += 1;
            }
        }
        // check if top-right outside
        if (garden.plot(right) != plant and garden.plot(above) != plant) {
            corners += 1;
        }
        // check if top-right inside
        {
            const diagonal = pos.up().right();
            if (garden.plot(right) == plant and garden.plot(above) == plant and garden.plot(diagonal) != plant) {
                corners += 1;
            }
        }
        // check if below-left outside
        if (garden.plot(left) != plant and garden.plot(below) != plant) {
            corners += 1;
        }
        // check if below-left inside
        {
            const diagonal = pos.down().left();
            if (garden.plot(left) == plant and garden.plot(below) == plant and garden.plot(diagonal) != plant) {
                corners += 1;
            }
        }
        // check if below-right outside
        if (garden.plot(right) != plant and garden.plot(below) != plant) {
            corners += 1;
        }
        // check if below-right inside
        {
            const diagonal = pos.down().right();
            if (garden.plot(right) == plant and garden.plot(below) == plant and garden.plot(diagonal) != plant) {
                corners += 1;
            }
        }
    }

    return corners;
}

fn findPrice(garden: *const Garden, regions: [][]Coordinates) usize {
    var total: usize = 0;
    for (regions) |region| {
        const area = region.len;
        const perimiter = findPerimeter(garden, region);
        total += area * perimiter;
    }
    return total;
}

fn findPrice2(garden: *const Garden, regions: [][]Coordinates) usize {
    var total: usize = 0;
    for (regions) |region| {
        const area = region.len;
        const sides = countSides(garden, region);
        total += area * sides;
    }
    return total;
}

test "finding regions" {
    const input =
        \\AAAA
        \\BBCD
        \\BBCC
        \\EEEC
    ;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const garden = try parseInput(input, allocator);
    const region = try findRegion(&garden, Coordinates{ .x = 0, .y = 0 }, allocator);
    try std.testing.expectEqual(4, region.len);

    const regions = try findAllRegions(&garden, allocator);
    try std.testing.expectEqual(5, regions.items.len);

    const perimeter_A = findPerimeter(&garden, regions.items[0]);
    try std.testing.expectEqual(10, perimeter_A);

    const perimeter_B = findPerimeter(&garden, regions.items[1]);
    try std.testing.expectEqual(8, perimeter_B);

    const perimeter_C = findPerimeter(&garden, regions.items[2]);
    try std.testing.expectEqual(10, perimeter_C);

    const perimeter_D = findPerimeter(&garden, regions.items[3]);
    try std.testing.expectEqual(4, perimeter_D);

    const price = findPrice(&garden, regions.items);
    try std.testing.expectEqual(140, price);

    // part 2
    const sides = countSides(&garden, regions.items[2]);
    try std.testing.expectEqual(8, sides);

    const price2 = findPrice2(&garden, regions.items);
    try std.testing.expectEqual(80, price2);
}

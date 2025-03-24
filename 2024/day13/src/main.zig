const std = @import("std");
const big = std.math.big;

const Button = struct {
    x: big.int.Managed,
    y: big.int.Managed,
};

const Prize = struct {
    x: big.int.Managed,
    y: big.int.Managed,
};

const Game = struct {
    a: Button,
    b: Button,
    prize: Prize,
};

pub fn main() !void {
    const input = @embedFile("input.txt");
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const games = try parseInput(input, allocator);

    // PART 1
    var total: usize = 0;
    for (games.items) |game| {
        if (try findOptimalSolution(game, allocator)) |tokens| {
            total += tokens;
        }
    }
    std.debug.print("PART 1: {}\n", .{total});

    // PART 2
    var total2: usize = 0;
    for (games.items) |game| {
        var new_prize = Prize{
            .x = try big.int.Managed.init(allocator),
            .y = try big.int.Managed.init(allocator),
        };
        try new_prize.x.addScalar(&game.prize.x, 10000000000000);
        try new_prize.y.addScalar(&game.prize.y, 10000000000000);
        const game2 = Game{
            .a = game.a,
            .b = game.b,
            .prize = new_prize,
        };
        if (try findOptimalSolution(game2, allocator)) |tokens| {
            total2 += tokens;
        }
    }

    std.debug.print("PART 2: {}\n", .{total2});
}

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Game) {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var games = std.ArrayList(Game).init(allocator);

    while (lines.peek() != null) {
        try games.append(Game{
            .a = try parseButton(lines.next().?, allocator),
            .b = try parseButton(lines.next().?, allocator),
            .prize = try parsePrize(lines.next().?, allocator),
        });
    }

    return games;
}

fn parseButton(input: []const u8, allocator: std.mem.Allocator) !Button {
    const x_start = std.mem.indexOf(u8, input, "X+").?;
    const x_end = std.mem.indexOfScalarPos(u8, input, x_start, ',').?;
    const x_value = std.fmt.parseInt(isize, input[x_start + 2 .. x_end], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    const y_start = std.mem.indexOf(u8, input, "Y+").?;
    const y_value = std.fmt.parseInt(isize, input[y_start + 2 ..], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    return Button{
        .x = try big.int.Managed.initSet(allocator, x_value),
        .y = try big.int.Managed.initSet(allocator, y_value),
    };
}

fn parsePrize(input: []const u8, allocator: std.mem.Allocator) !Prize {
    const x_start = std.mem.indexOf(u8, input, "X=").?;
    const x_end = std.mem.indexOfScalarPos(u8, input, x_start, ',').?;
    const x_value = std.fmt.parseInt(isize, input[x_start + 2 .. x_end], 10) catch |e| {
        std.debug.panic("parsing {s}: {any}", .{ input[x_start + 2 .. x_end], e });
    };

    const y_start = std.mem.indexOf(u8, input, "Y=").?;
    const y_value = std.fmt.parseInt(isize, input[y_start + 2 ..], 10) catch |e| {
        std.debug.panic("{any}", .{e});
    };

    return Prize{
        .x = try big.int.Managed.initSet(allocator, x_value),
        .y = try big.int.Managed.initSet(allocator, y_value),
    };
}

// Find solution. Solves set of 2 equations:
// X = A * a.x + B * b.x
// Y = A * a.y + B * b.y
// BigInt approach equivalent to:
//
// const det = game.a.x * game.b.y - game.a.y * game.b.x;
// if (det == 0) {
//     return null;
// }
//
// const a = @divTrunc(game.prize.x * game.b.y - game.prize.y * game.b.x, det);
// const b = @divTrunc(game.prize.y * game.a.x - game.prize.x * game.a.y, det);
fn findOptimalSolution(game: Game, allocator: std.mem.Allocator) !?usize {
    var det = try std.math.big.int.Managed.init(allocator);
    defer det.deinit();

    var temp = try std.math.big.int.Managed.init(allocator);
    defer temp.deinit();

    try det.mul(&game.a.x, &game.b.y);
    try temp.mul(&game.a.y, &game.b.x);
    try det.sub(&det, &temp);

    if (det.eqlZero()) {
        return null;
    }

    var a = try std.math.big.int.Managed.init(allocator);
    defer a.deinit();

    try a.mul(&game.prize.x, &game.b.y);
    try temp.mul(&game.prize.y, &game.b.x);
    try a.sub(&a, &temp);
    try a.divTrunc(&temp, &a, &det);
    if (!temp.eqlZero()) {
        return null;
    }

    var b = try std.math.big.int.Managed.init(allocator);
    defer b.deinit();

    try b.mul(&game.prize.y, &game.a.x);
    try temp.mul(&game.prize.x, &game.a.y);
    try b.sub(&b, &temp);
    try b.divTrunc(&temp, &b, &det);
    if (!temp.eqlZero()) {
        return null;
    }

    if (!a.isPositive() or !b.isPositive()) {
        return null;
    }

    return @intCast(try a.toInt(usize) * 3 + try b.toInt(usize));
}

test "parsing input" {
    const input =
        \\Button A: X+94, Y+34
        \\Button B: X+22, Y+67
        \\Prize: X=8400, Y=5400
        \\
        \\Button A: X+26, Y+66
        \\Button B: X+67, Y+21
        \\Prize: X=12748, Y=12176
        \\
        \\Button A: X+17, Y+86
        \\Button B: X+84, Y+37
        \\Prize: X=7870, Y=6450
        \\
        \\Button A: X+69, Y+23
        \\Button B: X+27, Y+71
        \\Prize: X=18641, Y=10279
    ;

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const games = try parseInput(input, allocator);
    try std.testing.expectEqual(4, games.items.len);

    try std.testing.expectEqual(280, try findOptimalSolution(games.items[0], allocator));
    try std.testing.expectEqual(null, try findOptimalSolution(games.items[1], allocator));

    // try std.testing.expectEqual(null, try findOptimalSolution(games.items[0], allocator));
    // try std.testing.expect(null != try findOptimalSolution(games.items[1], allocator));
    // try std.testing.expectEqual(null, try findOptimalSolution(games.items[2], allocator));
    // try std.testing.expect(null != try findOptimalSolution(games.items[3], allocator));
}

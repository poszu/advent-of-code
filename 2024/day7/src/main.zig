const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    const results = try solve(input);
    try stdout.print("[PART 1]: total = {d}\n", .{results[0]});
    try stdout.print("[PART 2]: total = {d}\n", .{results[1]});
}

fn solve(input: []const u8) !struct { usize, usize } {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    const allocator = std.heap.page_allocator;
    var total: usize = 0;
    var total2: usize = 0;
    while (lines.next()) |line| {
        const eq = try parseLine(line);
        const num_ops = eq.operands.items.len - 1;
        var operators = try operatorsGen.create(allocator, &[_]Operator{ .add, .multiply }, num_ops);
        if (solveEq(eq, &operators)) {
            total += eq.result;
        } else {
            var operators2 = try operatorsGen.create(allocator, &[_]Operator{ .add, .multiply, .concat }, num_ops);
            if (solveEq(eq, &operators2)) {
                total2 += eq.result;
            }
        }
    }
    return .{ total, total + total2 };
}

const Equation = struct {
    result: usize,
    operands: std.ArrayList(usize),
};

fn parseLine(line: []const u8) !Equation {
    const colonPos = std.mem.indexOfScalarPos(u8, line, 0, ':').?;
    const res = try std.fmt.parseInt(usize, line[0..colonPos], 10);
    var splitOps = std.mem.tokenizeScalar(u8, line[colonPos + 1 ..], ' ');

    var ops = std.ArrayList(usize).init(std.heap.page_allocator);
    while (splitOps.next()) |opStr| {
        const op = try std.fmt.parseInt(usize, opStr, 10);
        try ops.append(op);
    }

    return Equation{ .result = res, .operands = ops };
}

fn solveEq(eq: Equation, operators: *operatorsGen) bool {
    while (operators.next()) |ops| {
        var total: usize = eq.operands.items[0];
        for (ops, eq.operands.items[1..]) |operator, operand| {
            switch (operator) {
                .add => total += operand,
                .multiply => total *= operand,
                .concat => {
                    var op = operand;
                    while (op > 0) {
                        total *= 10;
                        op /= 10;
                    }
                    total += operand;
                },
            }
        }
        if (total == eq.result) {
            return true;
        }
    }
    return false;
}

const Operator = enum { add, multiply, concat };

const operatorsGen = struct {
    set: []const Operator,
    picked_indexes: std.ArrayList(usize),
    combination: std.ArrayList(Operator),
    done: bool,

    fn create(allocator: std.mem.Allocator, set: []const Operator, len: usize) !operatorsGen {
        return operatorsGen{
            .set = set,
            .combination = try std.ArrayList(Operator).initCapacity(allocator, len),
            .picked_indexes = try std.ArrayList(usize).initCapacity(allocator, len),
            .done = false,
        };
    }

    fn next(self: *operatorsGen) ?[]Operator {
        if (self.done) {
            return null;
        }
        if (self.combination.items.len == 0) {
            self.picked_indexes.appendNTimesAssumeCapacity(0, self.picked_indexes.capacity);
            self.combination.appendNTimesAssumeCapacity(self.set[0], self.combination.capacity);
            return self.combination.items;
        }
        const size = self.picked_indexes.items.len;
        var carried: usize = 0;
        for (0..size) |i| {
            const idx = (self.picked_indexes.items[i] + 1) % self.set.len;
            self.picked_indexes.items[i] = idx;
            self.combination.items[i] = self.set[idx];
            if (idx != 0) {
                break;
            }
            carried += 1;
        }
        if (carried == self.combination.items.len) {
            self.done = true;
            return null;
        }
        return self.combination.items;
    }
};

test "generating next combination" {
    const ops = [_]Operator{ .add, .multiply, .concat };
    var gen = try operatorsGen.create(std.heap.page_allocator, &ops, 3);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .add, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .add, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .add, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .multiply, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .multiply, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .multiply, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .concat, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .concat, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .concat, .add }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .add, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .add, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .add, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .multiply, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .multiply, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .multiply, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .concat, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .concat, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .concat, .multiply }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .add, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .add, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .add, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .multiply, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .multiply, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .multiply, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .add, .concat, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .multiply, .concat, .concat }, gen.next().?);
    try std.testing.expectEqualSlices(Operator, &[_]Operator{ .concat, .concat, .concat }, gen.next().?);
    try std.testing.expectEqual(null, gen.next());
}

test "parsing line" {
    const eq = try parseLine("21037: 9 7 18 13");
    defer eq.operands.deinit();
    try std.testing.expectEqual(21037, eq.result);
    try std.testing.expectEqualSlices(usize, &[_]usize{ 9, 7, 18, 13 }, eq.operands.items);
}

test "vector" {
    const input =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
        \\156: 15 6
        \\7290: 6 8 6 15
        \\161011: 16 10 13
        \\192: 17 8 14
        \\21037: 9 7 18 13
        \\292: 11 6 16 20
    ;
    const results = try solve(input);
    try std.testing.expectEqual(3749, results[0]);
    try std.testing.expectEqual(11387, results[1]);
}

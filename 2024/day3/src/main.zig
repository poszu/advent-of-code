const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    const total = mull_it_over(input);
    try stdout.print("[PART 1] total is {}\n", .{total});

    const total2 = mull_it_over2(input);
    try stdout.print("[PART 2] total is {}\n", .{total2});
}

fn mull_it_over(input: []const u8) usize {
    var total: usize = 0;
    var pos: usize = 0;

    while (std.mem.indexOfPos(u8, input, pos, "mul(")) |index| {
        pos = index + "mul(".len;
        var value = parse_number(input[pos..]) catch continue;
        const num1 = value.num;
        pos += value.digits;
        if (input[pos] != ',') {
            continue;
        }
        pos += 1;
        value = parse_number(input[pos..]) catch continue;
        const num2 = value.num;
        pos += value.digits;
        if (input[pos] != ')') {
            continue;
        }
        pos += 1;
        total += num1 * num2;
    }
    return total;
}

fn mull_it_over2(input: []const u8) usize {
    var total: usize = 0;
    var pos: usize = 0;
    var enabled = true;
    while (pos < input.len) {
        if (enabled and parse_dont(input[pos..])) {
            enabled = false;
            pos += "don't()".len;
            continue;
        }
        if (!enabled and parse_do(input[pos..])) {
            enabled = true;
            pos += "do()".len;
            continue;
        }
        if (!enabled) {
            pos += 1;
            continue;
        }
        if (!parse_mul(input[pos..])) {
            pos += 1;
            continue;
        }
        pos += "mul(".len;
        var value = parse_number(input[pos..]) catch continue;
        const num1 = value.num;
        pos += value.digits;
        if (input[pos] != ',') {
            pos += 1;
            continue;
        }
        pos += 1;
        value = parse_number(input[pos..]) catch continue;
        const num2 = value.num;
        pos += value.digits;
        if (input[pos] != ')') {
            pos += 1;
            continue;
        }
        pos += 1;
        total += num1 * num2;
    }
    return total;
}

fn parse_number(data: []const u8) std.fmt.ParseIntError!struct { num: usize, digits: usize } {
    var index: usize = 0;
    while (std.ascii.isDigit(data[index])) {
        index += 1;
    }
    if (index == 0) {
        return std.fmt.ParseIntError.InvalidCharacter;
    }
    const num = try std.fmt.parseInt(usize, data[0..index], 10);
    return .{ .num = num, .digits = index };
}

fn parse_mul(data: []const u8) bool {
    return std.mem.startsWith(u8, data, "mul(");
}

fn parse_dont(data: []const u8) bool {
    return std.mem.startsWith(u8, data, "don't()");
}

fn parse_do(data: []const u8) bool {
    return std.mem.startsWith(u8, data, "do()");
}

test "example vector" {
    const expect = std.testing.expect;
    const total = mull_it_over("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    try expect(total == 161);
}
test "example vector part2" {
    const expect = std.testing.expect;
    const total = mull_it_over2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    try expect(total == 48);
}

test "parses a number" {
    const expect = std.testing.expect;
    const val = try parse_number("132,");
    try expect(val.num == 132);
    try expect(val.digits == 3);
}

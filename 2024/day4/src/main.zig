const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    const data = try parse_input(input);
    defer data.deinit();

    try stdout.print("[PART 1] found XMAS {} times\n", .{count_xmas(data.items)});
    try stdout.print("[PART 2] found XMAS {} times\n", .{count_x_mas(data.items)});
}

fn count_xmas(data: [][]const u8) usize {
    var total: usize = 0;

    for (0..data.len) |y| {
        total += std.mem.count(u8, data[y], "XMAS");
        total += std.mem.count(u8, data[y], "SAMX");

        const width: usize = data[y].len;
        for (0..width) |x| {
            // look down
            if (y + 3 < data.len) {
                if (data[y][x] == 'X' and data[y + 1][x] == 'M' and data[y + 2][x] == 'A' and data[y + 3][x] == 'S') {
                    total += 1;
                }
                // look down backwards
                if (data[y][x] == 'S' and data[y + 1][x] == 'A' and data[y + 2][x] == 'M' and data[y + 3][x] == 'X') {
                    total += 1;
                }
            }
            // look diagonal right-down
            if (y + 3 < data.len and x + 3 < width) {
                if (data[y][x] == 'X' and data[y + 1][x + 1] == 'M' and data[y + 2][x + 2] == 'A' and data[y + 3][x + 3] == 'S') {
                    total += 1;
                }
                if (data[y][x] == 'S' and data[y + 1][x + 1] == 'A' and data[y + 2][x + 2] == 'M' and data[y + 3][x + 3] == 'X') {
                    total += 1;
                }
            }
            // look diagonal right-up
            if (y >= 3 and x + 3 < width) {
                if (data[y][x] == 'X' and data[y - 1][x + 1] == 'M' and data[y - 2][x + 2] == 'A' and data[y - 3][x + 3] == 'S') {
                    total += 1;
                }
                if (data[y][x] == 'S' and data[y - 1][x + 1] == 'A' and data[y - 2][x + 2] == 'M' and data[y - 3][x + 3] == 'X') {
                    total += 1;
                }
            }
        }
    }
    return total;
}

fn count_x_mas(data: [][]const u8) usize {
    var total: usize = 0;
    for (0..data.len - 2) |y| {
        const width = data[y].len;
        for (0..width - 2) |x| {
            if (data[y + 1][x + 1] != 'A') {
                continue;
            }
            if (!((data[y][x] == 'M' and data[y + 2][x + 2] == 'S') or (data[y][x] == 'S' and data[y + 2][x + 2] == 'M'))) {
                continue;
            }
            if (!((data[y + 2][x] == 'M' and data[y][x + 2] == 'S') or (data[y + 2][x] == 'S' and data[y][x + 2] == 'M'))) {
                continue;
            }
            total += 1;
        }
    }
    return total;
}

fn parse_input(input: []const u8) !std.ArrayList([]const u8) {
    var data = std.ArrayList([]const u8).init(std.heap.page_allocator);
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |l| {
        if (l.len > 0) {
            try data.append(l);
        }
    }
    return data;
}
test "simple test" {
    const input =
        \\..X...
        \\.SAMX.
        \\.A..A.
        \\XMAS.S
        \\.X....
    ;
    const list = try parse_input(input);
    defer list.deinit();
    const total = count_xmas(list.items);
    try std.testing.expectEqual(4, total);
}

test "test vector" {
    const input =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;
    const list = try parse_input(input);
    defer list.deinit();
    const total = count_xmas(list.items);
    try std.testing.expectEqual(18, total);
}

test "test vector part 2" {
    const input =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;
    const list = try parse_input(input);
    defer list.deinit();
    const total = count_x_mas(list.items);
    try std.testing.expectEqual(9, total);
}

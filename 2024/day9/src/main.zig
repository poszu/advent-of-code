const std = @import("std");

pub fn main() !void {
    // const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const disk_image = try scan_disk(input, allocator);
    const result = solve(disk_image.map.items, disk_image.total);

    std.debug.print("PART 1: {}\n", .{result});
    const result2 = try solve2(disk_image.map.items, allocator);
    std.debug.print("PART 2: {}\n", .{result2});
}

const Cell = struct {
    addr: usize,
    id: ?usize,
    len: usize,

    fn new_file(addr: usize, len: usize, id: usize) Cell {
        return Cell{ .addr = addr, .len = len, .id = id };
    }
    fn new_free(addr: usize, len: usize) Cell {
        return Cell{ .addr = addr, .len = len, .id = null };
    }
};

fn scan_disk(input: []const u8, allocator: std.mem.Allocator) !struct { map: std.ArrayList(Cell), total: usize } {
    var memory = std.ArrayList(Cell).init(allocator);
    var total_blocks: usize = 0;
    var is_file = true;
    var id: usize = 0;
    var addr: usize = 0;
    for (input) |c| {
        if (c == '\n') {
            break;
        }
        const d = try std.fmt.charToDigit(c, 10);
        if (is_file) {
            try memory.append(Cell.new_file(addr, d, id));
            id += 1;
            total_blocks += d;
        } else {
            try memory.append(Cell.new_free(addr, d));
        }
        is_file = !is_file;
        addr += d;
    }
    if (is_file) {
        // pop last free cell
        _ = memory.pop();
    }
    return .{ .map = memory, .total = total_blocks };
}

fn solve(memory: []const Cell, total_blocks: usize) usize {
    var idx: usize = 0;
    var left: usize = 0;
    var right = memory.len - 1;
    var checksum: usize = 0;
    var idx_in_cell: usize = 0;
    var idx_in_right_cell: usize = 0;
    while (idx < total_blocks) {
        const cell = memory[left];
        if (cell.id) |id| {
            checksum += idx * id;
        } else {
            if (cell.len == 0) {
                left += 1;
                idx_in_cell = 0;
                continue;
            }
            // pick from the end
            const right_file = memory[right];
            checksum += idx * right_file.id.?;

            idx_in_right_cell = (idx_in_right_cell + 1) % right_file.len;
            if (idx_in_right_cell == 0) {
                right -= 2; // skip next 'free'.
            }
        }

        idx_in_cell = (idx_in_cell + 1) % cell.len;
        if (idx_in_cell == 0) {
            left += 1;
        }
        idx += 1;
    }

    return checksum;
}

fn solve2(cells: []const Cell, allocator: std.mem.Allocator) !usize {
    var memory = std.hash_map.AutoHashMap(usize, Cell).init(allocator);
    defer memory.deinit();

    var addr: usize = 0;
    for (cells) |c| {
        try memory.put(addr, c);
        addr += c.len;
    }
    // move files
    var right = cells.len - 1;

    while (right > 0) {
        var file = cells[right];
        addr = 0;
        while (addr < file.addr) {
            const cell = memory.get(addr).?;
            if (cell.id == null) {
                if (cell.len >= file.len) {
                    // replace with empty cell
                    try memory.put(file.addr, Cell.new_free(file.addr, file.len));

                    // put into the new position
                    file.addr = addr;
                    try memory.put(addr, file);

                    // insert cell for the remaining empty space
                    const left_space = cell.len - file.len;
                    if (left_space > 0) {
                        try memory.put(addr + file.len, Cell.new_free(addr + file.len, left_space));
                    }
                    break;
                }
            }
            addr += cell.len;
        }
        right -= 2;
    }

    var checksum: usize = 0;
    var iterator = memory.iterator();
    while (iterator.next()) |entry| {
        if (entry.value_ptr.id) |id| {
            for (0..entry.value_ptr.len) |i| {
                checksum += (entry.key_ptr.* + i) * id;
            }
        }
        addr += entry.value_ptr.len;
    }

    return checksum;
}

test "part 1" {
    const disk_image = try scan_disk("2333133121414131402", std.testing.allocator);
    const result = solve(disk_image.map.items, disk_image.total);
    disk_image.map.deinit();
    try std.testing.expectEqual(1928, result);
}

test "part 2" {
    const disk_image = try scan_disk("2333133121414131402", std.testing.allocator);
    const result = solve2(disk_image.map.items, std.testing.allocator);
    disk_image.map.deinit();
    try std.testing.expectEqual(2858, result);
}

const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input.txt");

    var parsed = try parseInput(input, std.heap.page_allocator);
    defer parsed.program.deinit();

    const instructions = try parseProgram(parsed.program.items, std.heap.page_allocator);
    defer instructions.deinit();
    std.debug.print("Instructions: {any}\n", .{instructions.items});

    var cpu = parsed.cpu;
    const out = try cpu.executeProgram(instructions.items, std.heap.page_allocator);
    defer out.deinit();
    std.debug.print("PART 1: {any}\n", .{out.items});

    // PART 2
    const solution = find(parsed.program.items, 0).?;
    // make sure it worked
    cpu.reset(solution);
    const out2 = try cpu.executeProgram(instructions.items, std.heap.page_allocator);
    defer out2.deinit();
    if (std.mem.eql(usize, out2.items, parsed.program.items)) {
        std.debug.print("PART 2: {}\n", .{solution});
    } else {
        std.debug.print("PART 2: found wrong solution {} with output {any}\n", .{ solution, out2.items });
    }
}

const Opcode = enum { adv, bxl, bst, jnz, bxc, out, bdv, cdv };

const Instruction = struct {
    opcode: Opcode,
    operand: usize,

    pub fn format(
        self: Instruction,
        comptime _: []const u8,
        _: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        const getOperandStr = switch (self.operand) {
            0, 1, 2, 3 => try std.fmt.allocPrint(std.heap.page_allocator, "{}", .{self.operand}),
            4 => "A",
            5 => "B",
            6 => "C",
            else => try std.fmt.allocPrint(std.heap.page_allocator, "invalid({})", .{self.operand}),
        };

        // For numeric operands, we need to free the allocated memory
        defer if (self.operand < 4 or self.operand > 6) {
            std.heap.page_allocator.free(getOperandStr);
        };

        switch (self.opcode) {
            Opcode.adv => try writer.print("A = A / 2^{s}", .{getOperandStr}),
            Opcode.bxl => try writer.print("B = B ^ {}", .{self.operand}),
            Opcode.bst => try writer.print("B = {s} % 8", .{getOperandStr}),
            Opcode.jnz => try writer.print("if (A != 0) goto {}", .{self.operand}),
            Opcode.bxc => try writer.print("B = B ^ C", .{}),
            Opcode.out => try writer.print("output({s} % 8)", .{getOperandStr}),
            Opcode.bdv => try writer.print("B = A / 2^{s}", .{getOperandStr}),
            Opcode.cdv => try writer.print("C = A / 2^{s}", .{getOperandStr}),
        }
    }
};

const CPU = struct {
    instruction_ptr: usize,
    registers: [3]usize,

    fn reset(self: *CPU, a: usize) void {
        self.instruction_ptr = 0;
        self.registers = [_]usize{ a, 0, 0 };
    }

    fn executeProgram(self: *CPU, program: []const Instruction, allocator: std.mem.Allocator) !std.ArrayList(usize) {
        var out = std.ArrayList(usize).init(allocator);
        self.instruction_ptr = 0;

        while (self.instruction_ptr < program.len) {
            const instruction = program[self.instruction_ptr];
            self.instruction_ptr += 1;
            if (self.executeInstruction(instruction)) |value| {
                try out.append(value);
            }
        }

        return out;
    }

    fn executeInstruction(self: *CPU, instruction: Instruction) ?usize {
        var result: ?usize = null;
        switch (instruction.opcode) {
            Opcode.adv => {
                const numerator = self.registers[0];
                const denom_power = self.combo_operand(instruction.operand);
                self.registers[0] = numerator / std.math.pow(usize, 2, denom_power);
            },
            Opcode.bxl => {
                self.registers[1] ^= instruction.operand;
            },
            Opcode.bst => {
                self.registers[1] = self.combo_operand(instruction.operand) % 8;
            },
            Opcode.jnz => {
                if (self.registers[0] != 0) {
                    self.instruction_ptr = instruction.operand / 2;
                }
            },
            Opcode.bxc => {
                self.registers[1] ^= self.registers[2];
            },
            Opcode.out => {
                result = self.combo_operand(instruction.operand) % 8;
            },
            Opcode.bdv => {
                const numerator = self.registers[0];
                const denom_power = self.combo_operand(instruction.operand);
                self.registers[1] = numerator / std.math.pow(usize, 2, denom_power);
            },
            Opcode.cdv => {
                const numerator = self.registers[0];
                const denom_power = self.combo_operand(instruction.operand);
                self.registers[2] = numerator / std.math.pow(usize, 2, denom_power);
            },
        }
        return result;
    }

    fn combo_operand(self: *const CPU, op: usize) usize {
        return switch (op) {
            0, 1, 2, 3 => op,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            else => unreachable,
        };
    }
};

fn find(program: []const usize, answer: usize) ?usize {
    if (program.len == 0) {
        return answer;
    }
    for (0..8) |i| {
        // reconstructed program from the input.txt
        var b = i;
        const a = (answer << 3) + b;
        b = b ^ 5;
        const c = a / std.math.pow(usize, 2, b);
        b = b ^ 6;
        b = b ^ c;
        if ((b % 8) == program[program.len - 1]) {
            if (find(program[0 .. program.len - 1], a)) |ans| {
                return ans;
            }
        }
    }
    return null;
}

fn parseInput(input: []const u8, allocator: std.mem.Allocator) !struct { cpu: CPU, program: std.ArrayList(usize) } {
    var registers = [3]usize{ 0, 0, 0 };

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    for (0..3) |i| {
        const line = lines.next().?;
        const value_index = std.mem.indexOfScalar(u8, line, ':').?;
        const reg = try std.fmt.parseInt(usize, line[value_index + 2 ..], 10);
        registers[i] = reg;
    }

    const prog_line = lines.next().?;
    const instructions_index = std.mem.indexOfScalar(u8, prog_line, ':').? + 2;

    var program = std.ArrayList(usize).init(allocator);
    var values = std.mem.splitScalar(u8, prog_line[instructions_index..], ',');
    while (values.next()) |value| {
        try program.append(try std.fmt.parseInt(usize, value, 10));
    }

    return .{
        .cpu = CPU{ .instruction_ptr = 0, .registers = registers },
        .program = program,
    };
}

fn parseProgram(prog: []const usize, allocator: std.mem.Allocator) !std.ArrayList(Instruction) {
    var program = std.ArrayList(Instruction).init(allocator);
    var i: usize = 0;
    while (i < prog.len) : (i += 2) {
        try program.append(Instruction{
            .opcode = @enumFromInt(prog[i]),
            .operand = prog[i + 1],
        });
    }
    return program;
}

test "part 1" {
    const input =
        \\Register A: 729
        \\Register B: 0
        \\Register C: 0
        \\
        \\Program: 0,3,5,4,3,0
    ;

    var parsed = try parseInput(input, std.testing.allocator);
    defer parsed.program.deinit();

    try std.testing.expectEqual(729, parsed.cpu.registers[0]);
    try std.testing.expectEqual(0, parsed.cpu.registers[1]);
    try std.testing.expectEqual(0, parsed.cpu.registers[2]);

    const instructions = try parseProgram(parsed.program.items, std.heap.page_allocator);
    defer instructions.deinit();

    var cpu = parsed.cpu;
    const out = try cpu.executeProgram(instructions.items, std.heap.page_allocator);
    defer out.deinit();
    std.debug.print("PART 1: {any}\n", .{out.items});
}

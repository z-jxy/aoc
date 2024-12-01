const std = @import("std");
const input = @embedFile("../inputs/03.test");

const DIRECTIONAL_TUPLE = struct { isize, isize };

const DIRECTIONS: [8]DIRECTIONAL_TUPLE = .{
    .{ -1, -1 },
    .{ -1, 0 },
    .{ -1, 1 },
    .{ 0, -1 },
    .{ 0, 1 },
    .{ 1, -1 },
    .{ 1, 0 },
    .{ 1, 1 },
};

pub fn main() !void {
    std.debug.print("hey!\n", .{});
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = general_purpose_allocator.allocator();

    //var grid: [][]const u8 = undefined;

    var rows = std.mem.tokenizeSequence(u8, input, "\n");

    var num_rows: usize = 0;
    while (rows.next()) |_| {
        num_rows += 1;
    }

    // Allocate memory for the grid
    var grid = try gpa.alloc([]const u8, num_rows);
    defer gpa.free(grid);

    // Fill the grid
    rows.reset();
    var rowIndex: usize = 0;
    while (rows.next()) |row| {
        grid[rowIndex] = row;
        rowIndex += 1;
    }

    rows.reset();
    rowIndex = 0;
    var colIndex: usize = 0;

    while (rows.next()) |row| {
        rowIndex += 1;
        var cols = std.mem.tokenizeSequence(u8, row, "\n");
        while (cols.next()) |col| {
            colIndex += 1;
            const chars = col[0..col.len];
            for (chars) |ch| {
                if (ch != '.') {
                    const is_ascii_punctuation = ch >= 0x21 and ch <= 0x2F;

                    if (is_ascii_punctuation) {
                        inline for (DIRECTIONS) |dir| {
                            var x: isize = dir[0];
                            var y: isize = dir[1];

                            var tr: isize = @intCast(rowIndex);
                            tr += x;
                            var target_row: usize = @intCast(tr);

                            var tc: isize = @intCast(colIndex);
                            tc += y;
                            var target_col: usize = @intCast(tc);

                            if (target_row >= 0 and target_row < grid.len and target_col < grid[target_row].len) {
                                const char_at_pos = grid[target_row][target_col];
                                if (std.ascii.isDigit(char_at_pos)) {
                                    std.debug.print("digit! {d} {d} ||: {c}\n", .{ x, y, char_at_pos });
                                }
                                std.debug.print("{d} {d} ||: {c}\n", .{ x, y, char_at_pos });
                            }

                            //std.debug.print("{d} {d} ||: {c}\n", .{ x, y, char_at_pos });

                            //std.debug.print("{d} {d} ||: {c}\n", .{ x, y, ch });
                        }
                    }
                }

                std.debug.print("{c}", .{ch});
            }
        }

        std.debug.print("\n", .{});
    }
}

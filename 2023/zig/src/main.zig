const std = @import("std");
const input = @embedFile("../inputs/02.txt");
const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn getPowerOfRound(rounds: *std.mem.SplitIterator(u8, std.mem.DelimiterType.any)) !u32 {
    var max_red: u32 = 0;
    var max_green: u32 = 0;
    var max_blue: u32 = 0;

    while (rounds.next()) |round| {
        var cubes_slice = std.mem.splitAny(u8, round, ",");
        while (cubes_slice.next()) |round_cubes| {
            var cube_split = std.mem.splitAny(u8, std.mem.trim(u8, round_cubes, " "), " ");
            const count = try std.fmt.parseInt(u8, cube_split.next().?, 10);
            const cube = cube_split.next().?;

            if (std.mem.eql(u8, cube, "red")) {
                max_red = @max(max_red, count);
            } else if (std.mem.eql(u8, cube, "blue")) {
                max_blue = @max(max_blue, count);
            } else if (std.mem.eql(u8, cube, "green")) {
                max_green = @max(max_green, count);
            }
        }
    }
    return max_red * max_green * max_blue;
}

fn processCubes(cubes: []const u8) !bool {
    var cube_info_slice = std.mem.splitAny(u8, std.mem.trim(u8, cubes, " "), " ");
    const count = try std.fmt.parseInt(u8, cube_info_slice.next().?, 10);
    const cube = cube_info_slice.next().?;

    return (std.mem.eql(u8, cube, "red") and count <= MAX_RED) or
        (std.mem.eql(u8, cube, "green") and count <= MAX_GREEN) or
        (std.mem.eql(u8, cube, "blue") and count <= MAX_BLUE);
}

fn processRounds(rounds: []const u8) !bool {
    var cubes_slice = std.mem.splitAny(u8, rounds, ",");
    while (cubes_slice.next()) |round_cubes| {
        if (!try processCubes(round_cubes)) {
            return false;
        }
    }
    return true;
}

pub fn main() !void {
    var lines = std.mem.tokenize(u8, input, "\n");
    var game_id_sum: u32 = 0;
    var power_sum: u32 = 0;
    while (lines.next()) |line| {
        var line_split = std.mem.splitAny(u8, line, ":");
        const gameId: u8 = try std.fmt.parseInt(u8, line_split.next().?[5..], 10);
        var possible = true;
        var rounds_split = std.mem.splitAny(u8, line_split.next().?, ";");
        while (rounds_split.next()) |part| {
            possible = try processRounds(part);
            if (!possible) {
                break;
            }
        }
        if (possible) {
            game_id_sum += gameId;
        }

        rounds_split.reset();

        power_sum += try getPowerOfRound(&rounds_split);
    }

    std.debug.print("Part 1: {d}\nPart 2: {d}\n", .{ game_id_sum, power_sum });
}

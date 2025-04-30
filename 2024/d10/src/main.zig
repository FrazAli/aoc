const std = @import("std");

const stdout = std.io.getStdOut().writer();
const print = std.debug.print;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();
const ArrayList = std.ArrayList;

const Tuple = struct { i64, i64 };
const Directions = [4]Tuple{ .{ -1, 0 }, .{ 1, 0 }, .{ 0, -1 }, .{ 0, 1 } };

fn readMap() ![][]u8 {
    const stdin = std.io.getStdIn().reader();

    var buf: [9]u8 = undefined; // we expect lines of 9 chars including the newline
    var list = ArrayList([]u8).init(allocator);

    while (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const bytes = try allocator.alloc(u8, line.len);
        for (0.., line) |i, ch| {
            print("ch: {c}, ascii: {d}\n", .{ ch, ch });
            bytes[i] = ch - '0';
            print(">> ch: {c}, ascii: {d}\n", .{ bytes[i], bytes[i] });
        }

        try list.append(bytes);
    }

    return list.items;
}

fn isBounded(pos: Tuple, map: [][]u8) bool {
    return 0 <= pos[0] and pos[0] < map.len and 0 <= pos[1] and pos[1] < map[0].len;
}

fn tuple2Add(a: Tuple, b: Tuple) Tuple {
    return .{ a[0] + b[0], a[1] + b[1] };
}

fn mapGet(map: [][]u8, pos: Tuple) u8 {
    return map[@intCast(pos[0])][@intCast(pos[1])];
}

fn hike(map: [][]u8, pos: Tuple) ![]Tuple {
    if (mapGet(map, pos) == 9) {
        const xs = try allocator.alloc(Tuple, 1);
        xs[0] = pos;
        return xs;
    } else {
        var list = ArrayList(Tuple).init(allocator);
        for (Directions) |dir| {
            const newPos = tuple2Add(pos, dir);
            if (isBounded(newPos, map) and mapGet(map, pos) + 1 == mapGet(map, newPos)) {
                const results = try hike(map, newPos);
                try list.appendSlice(results);
            }
        }

        return list.items;
    }
}

fn TrailHead(map: [][]u8, row: i64, col: i64) !u64 {
    const results = try hike(map, .{ row, col });

    var dups = std.AutoHashMap(Tuple, bool).init(allocator);
    for (results) |result| {
        try dups.put(result, true);
    }

    return dups.count();
}

fn raiting(map: [][]u8, row: i64, col: i64) !u64 {
    const results = try hike(map, .{ row, col });

    return results.len;
}

fn part1(map: [][]u8) !u64 {
    var total: u64 = 0;

    for (0..map.len) |row| {
        for (0..map[row].len) |col| {
            if (map[row][col] == 0) {
                total += try TrailHead(map, @intCast(row), @intCast(col));
            }
        }
    }

    return total;
}

fn part2(map: [][]u8) !u64 {
    var total: u64 = 0;

    for (0..map.len) |row| {
        for (0..map[row].len) |col| {
            if (map[row][col] == 0) {
                total += try raiting(map, @intCast(row), @intCast(col));
            }
        }
    }

    return total;
}

pub fn main() !void {
    const topo_map = try readMap();

    const r1 = try part1(topo_map);
    try stdout.print("Part 1: {}\n", .{r1});

    const r2 = try part2(topo_map);
    try stdout.print("Part 2: {}\n", .{r2});
}

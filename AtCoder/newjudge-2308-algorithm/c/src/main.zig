const std = @import("std");

pub fn main() !void {
    var buffer: [1000000]u8 = undefined;
    const stdin = std.io.getStdIn();
    const reader = stdin.reader();
    const stdout = std.io.getStdOut();
    const writer = stdout.writer();
    const N_ = try reader.readUntilDelimiterOrEof(&buffer, '\n');
    const N = try std.fmt.parseInt(usize, N_.?, 10);

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var map = std.AutoHashMap(usize, void).init(allocator);
    defer map.deinit();

    var i: usize = 1;
    while (i <= N * 2 + 1) : (i += 1) {
        if (map.contains(i)) {
            continue;
        }
        try writer.print("{d}\n", .{i});
        try map.put(i, {});
        const x_ = try reader.readUntilDelimiterOrEof(&buffer, '\n');
        const x = try std.fmt.parseInt(usize, x_.?, 10);
        if (x == 0) {
            break;
        }
        try map.put(x, {});
    }
}

const std = @import("std");

pub fn main() !void {
    var buffer: [1000000]u8 = undefined;
    const stdin = std.io.getStdIn();
    const reader = stdin.reader();
    const stdout = std.io.getStdOut();
    const L_ = try reader.readUntilDelimiter(&buffer, ' ');
    const L = try std.fmt.parseInt(usize, L_, 10);
    const R_ = try reader.readUntilDelimiterOrEof(&buffer, '\n');
    const R = try std.fmt.parseInt(usize, R_.?, 10);

    try stdout.writer().print("{s}\n", .{"atcoder"[L - 1 .. R]});
}

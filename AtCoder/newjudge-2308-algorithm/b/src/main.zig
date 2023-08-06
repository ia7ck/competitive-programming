const std = @import("std");

fn pow(base: u64, exp: usize) u64 {
    var result: u64 = 1;
    var i: usize = 0;
    while (i < exp) : (i += 1) {
        result *= base;
    }
    return result;
}

pub fn main() !void {
    var buffer: [1000000]u8 = undefined;
    const stdin = std.io.getStdIn();
    const reader = stdin.reader();
    const stdout = std.io.getStdOut();
    const X_ = try reader.readUntilDelimiter(&buffer, ' ');
    var X = try std.fmt.parseInt(u64, X_, 10);
    const K_ = try reader.readUntilDelimiterOrEof(&buffer, '\n');
    const K = try std.fmt.parseInt(usize, K_.?, 10);

    var i: usize = 0;
    while (i < K) : (i += 1) {
        // 10^iの位以下を四捨五入する
        const p = pow(10, i);
        X /= p;
        const r = X % 10;
        if (r <= 4) {
            X = (X - r) * p;
        } else {
            X = (X - r + 10) * p;
        }
    }
    try stdout.writer().print("{d}\n", .{X});
}

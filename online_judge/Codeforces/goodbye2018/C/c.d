void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.range;

  // for (int n = 2; n <= 20; n++) {
  // auto ids = iota(n);
  // int[] ans;
  // for (int k = 1; k <= n; k++) {
  // int cur = k, s = 1;
  // while (cur % n) {
  // s += (cur + 1);
  // cur += k;
  // cur %= n;
  // }
  // ans ~= s;
  // }
  // ans = ans.sort.uniq.array;
  // writeln(n, " ", ans);
  // }

  long n;
  rd(n);
  long[] ans;
  for (long i = 1; i * i <= n; i++) {
    if (n % i == 0) {
      auto j = n / i;
      ans ~= i * (2 + (i - 1) * j) / 2;
      ans ~= j * (2 + (j - 1) * i) / 2;
    }
  }
  ans = ans.sort.uniq.array;
  writefln("%(%s %)", ans);
}

// 16
// 1 10 28 64 136
// 1  2  4  8  16
// 1 = 1
// 10 = 1 + 9
// 28 = 1 + 5 + 9 + 13
// 64 = 1 + 3 + 5 + 7 + 9 + 11 + 13 + 15

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

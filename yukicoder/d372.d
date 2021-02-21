void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  int m;
  rd(m);
  const int mod = 10 ^^ 9 + 7;

  int zero = 0;
  auto dp = new int[][](2, m);
  foreach (i; 0 .. s.length) {
    auto cur = i % 2, nex = cur ^ 1;
    auto curDP = dp[cur], nexDP = dp[nex];
    auto d = s[i] - '0';
    foreach (j; 0 .. m) {
      nexDP[j] = curDP[j];
    }
    foreach (j; 0 .. m) {
      (nexDP[(j * 10 + d) % m] += curDP[j]) %= mod;
    }
    if (d == 0) {
      (zero += 1) %= mod;
    } else {
      (nexDP[d % m] += 1) %= mod;
    }
  }
  writeln((dp[(s.length) % 2][0] + zero) % mod);
}

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

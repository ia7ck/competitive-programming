void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto s = readln.chomp.to!(char[]);
  auto a = readln.split.to!(long[]);

  const char[] pat = "hard";
  auto dp = new long[](pat.length + 1);
  fill(dp, 1_000_000_000_000_000_000);
  dp[0] = 0;
  foreach (i; 0 .. n) {
    auto nex = new long[](pat.length + 1);
    fill(nex, 1_000_000_000_000_000_000);
    foreach (j, c; pat) {
      if (s[i] == c) {
        nex[j] = min(nex[j], dp[j] + a[i]); // s[i]を消す
        nex[j + 1] = min(nex[j + 1], dp[j]); // 消さない
      } else {
        nex[j] = min(nex[j], dp[j]); // 消さない
      }
    }
    dp.swap(nex);
  }
  writeln(reduce!(min)(dp[0 .. (pat.length)]));
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

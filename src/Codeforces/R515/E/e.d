void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n, m;
  rd(n, m);
  auto a = readln.chomp
    .to!(char[])
    .map!((ch) => (ch - '0'))
    .array
    .to!(long[]);
  auto b = readln.chomp
    .to!(char[])
    .map!((ch) => (ch - '0'))
    .array
    .to!(long[]);

  if (n < m) {
    a.reverse;
    foreach (_; 0 .. (m - n))
      a ~= 0;
    a.reverse;
  } else {
    b.reverse;
    foreach (_; 0 .. (n - m))
      b ~= 0;
    b.reverse;
  }

  foreach (i; 1 .. b.length)
    b[i] += b[i - 1];
  const long mod = 998244353;
  long pow2 = 1, ans = 0;
  foreach_reverse (i; 0 .. a.length) {
    if (a[i]) {
      ans += pow2 * b[i] % mod;
      ans %= mod;
    }
    pow2 *= 2;
    pow2 %= mod;
  }
  writeln(ans);

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

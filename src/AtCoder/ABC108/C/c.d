void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, k;
  rd(n, k);
  long tot = 0;
  for (long a = 1; a <= n; a++) {
    if ((a * 2) % k == 0) {
      auto x = (a + n) / k - (a + 1 + k - 1) / k + 1;
      tot += x * x;
    }
  }
  writeln(tot);

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

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  int ans = n & 1;
  n -= n & 1;
  for (int i = 2, ad = 4; i < n; i += 2, ad += 4) {
    ans += ad;
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

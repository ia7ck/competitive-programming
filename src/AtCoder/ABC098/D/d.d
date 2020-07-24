void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  long ans = 0;
  for (int i = 0, j = 0, s = 0; i < n; s ^= a[i++]) {
    while (j < n && ((s & a[j]) == 0)) {
      s ^= a[j++];
      ans += (j - i);
    }
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

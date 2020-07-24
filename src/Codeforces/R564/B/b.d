void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.math : abs;

  int n;
  rd(n);

  auto m = n / 2 + 1;
  writeln(m);
  for (int i = 1, r = 1, c = 1; i <= n; i++) {
    writeln(r, " ", c);
    if (i & 1) {
      c++;
    } else {
      r++;
    }
  }

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

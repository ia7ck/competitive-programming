void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int y, b, r;
  rd(y, b, r);

  int mx = 0;
  for (int i = 1; i <= y; i++) {
    for (int j = 1; j <= b; j++) {
      for (int k = 1; k <= r; k++) {
        if (i + 1 == j && j + 1 == k) {
          mx = max(mx, i + j + k);
        }
      }
    }
  }
  writeln(mx);
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

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);

  for (int i = 0; i <= 100; i++) {
    for (int j = 0; j <= 100; j++) {
      if (i * 4 + j * 7 == n) {
        writeln("Yes");
        return;
      }
    }
  }
  writeln("No");
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

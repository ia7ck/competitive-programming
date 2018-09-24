void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m, x, y;
  rd(n, m, x, y);
  auto a = readln.split.to!(int[]);
  auto b = readln.split.to!(int[]);

  for (int z = x + 1; z <= y; z++) {
    auto ok = reduce!((r, e) => (r && e < z))(true, a) && reduce!((r, e) => (r && e >= z))(true, b);
    if (ok) {
      writeln("No War");
      return;
    }
  }
  writeln("War");
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

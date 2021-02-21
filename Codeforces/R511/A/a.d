void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  if (n % 3 < 2) {
    writeln(1, " ", 1, " ", n - 2);
  } else {
    writeln(1, " ", 2, " ", n - 3);
  }
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  if (reduce!"a+b"(a) > 0) {
    writeln("HARD");
  } else {
    writeln("EASY");
  }
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

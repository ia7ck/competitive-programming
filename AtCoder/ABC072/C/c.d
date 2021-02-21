void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n;
  rd(n);
  auto a = readln.split
    .to!(int[])
    .map!((e) => (e + 1))
    .array;

  int M = 100000 + 5;
  auto map = new int[](M);
  foreach (e; a) {
    for (int d = -1; d <= 1; d++) {
      map[e + d]++;
    }
  }
  writeln(reduce!(max)(map));
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

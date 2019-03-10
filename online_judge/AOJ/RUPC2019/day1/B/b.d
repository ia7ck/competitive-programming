void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n;
  rd(n);

  auto a = readln.split.map!((s) => (s == "T" ? 1 : 0)).array;
  int f(int x, int y) {
    return (x - y <= 0);
  }
  //
  auto t = f(a[0], a[1]);
  foreach (i; 2 .. n) {
    t = f(t, a[i]);
  }
  if (t) {
    writeln("T");
  } else {
    writeln("F");
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

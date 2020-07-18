void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto a = readln.split.to!(int[]);
  auto ch = ['A', 'B', 'C'];

  auto mx = a.reduce!(max);
  foreach (i; 0 .. 3) {
    if (a[i] == mx) {
      writeln(ch[i]);
      return;
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

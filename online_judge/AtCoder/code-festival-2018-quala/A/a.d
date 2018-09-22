void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int a, b, c, s;
  rd(a);
  rd(b);
  rd(c);
  rd(s);

  auto t = a + b + c;
  if (s == t || s == t + 1 || s == t + 2 || s == t + 3) {
    writeln("Yes");
  } else {
    writeln("No");
  }
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

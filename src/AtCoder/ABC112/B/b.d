void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, t;
  rd(n, t);
  int mn = 10 ^^ 9;
  foreach (_; 0 .. n) {
    int c, tm;
    rd(c, tm);
    if (tm <= t) {
      mn = min(mn, c);
    }
  }
  if (mn < 10 ^^ 9) {
    writeln(mn);
  } else {
    writeln("TLE");
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

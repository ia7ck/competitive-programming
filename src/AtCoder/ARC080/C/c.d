void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  int o, t, f;
  foreach (e; a) {
    if (e % 4 == 0)
      f++;
    else if (e % 2 == 0)
      t++;
    else
      o++;
  }
  t %= 2;
  o += t;
  if (o <= f + 1) {
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

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  int one = 0, two = 0;
  foreach (el; a) {
    if (el == 1) {
      one += 1;
    } else {
      two += 1;
    }
  }
  if (one == 0 || two == 0) {
    writefln("%(%s %)", a);
    return;
  }
  int[] b;
  b ~= [2, 1];
  foreach (_; 0 .. (two - 1)) {
    b ~= 2;
  }
  foreach (_; 0 .. (one - 1)) {
    b ~= 1;
  }
  writefln("%(%s %)", b);
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

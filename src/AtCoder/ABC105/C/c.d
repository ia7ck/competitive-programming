void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  // 1 -2 4 -8 16 -32
  long n;
  rd(n);
  if (n == 0) {
    writeln(0);
    return;
  }
  int[] a;
  for (long k = 1; n != 0; k *= -2) {
    if (n % (k * (-2)) != 0) {
      a ~= 1;
      n = n - k;
    } else {
      a ~= 0;
    }
  }
  reverse(a);
  writefln("%(%s%)", a);
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

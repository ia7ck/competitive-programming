void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.math : abs;

  int x, y, z;
  rd(x, y, z);

  bool plus = false, minus = false, zero = false;
  if (x + z > y) {
    plus = true;
  }
  if (x < y + z) {
    minus = true;
  }
  if (z >= abs(x - y)) {
    if ((z - abs(x - y)) % 2 == 0) {
      zero = true;
    }
  }
  if (plus && !minus && !zero) {
    writeln("+");
  } else if (!plus && minus && !zero) {
    writeln("-");
  } else if (!plus && !minus && zero) {
    writeln("0");
  } else {
    writeln("?");
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

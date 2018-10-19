void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto n = readln.chomp.to!(char[]);
  auto k = readln.chomp.to!(char[]);

  int s = 0;
  foreach (c; n) {
    s = (s * 10 + (c - '0')) % 6;
  }
  if (s == 2) {
    if ((k[$ - 1] - '0') & 1) {
      writeln(8);
    } else {
      writeln(7);
    }
  } else if (s == 5) {
    if ((k[$ - 1] - '0') & 1) {
      writeln(1);
    } else {
      writeln(2);
    }
  } else {
    writeln("428571"[s]);
  }

}

// 28,57,14 28,5714 2857,14 285714 285714 28,5714
// 2857,14 285714 2857,14

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

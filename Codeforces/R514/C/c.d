void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  int g = 1;
  int[] ans;
  while (n) {
    if (n == 3) {
      ans ~= g;
      ans ~= g;
      ans ~= g * 3;
      break;
    }
    int k = (n + 1) / 2;
    n -= k;
    foreach (_; 0 .. k)
      ans ~= g;
    g *= 2;
  }
  writefln("%(%s %)", ans);

}

// 5 => 1 1 1 2 4
// 6 => 1 1 1 2 2 6
// 7 => 1 1 1 1 2 2 6
// 8 => 1 1 1 1 2 2 4 8
// 10 => 1 1 1 1 1 2 2 2 4 8
// 12 => 1 1 1 1 1 1 2 2 2 4 4 12

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

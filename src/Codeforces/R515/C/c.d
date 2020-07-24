void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int q;
  rd(q);
  int l = 0, r = 0; // [l, r)
  auto pos = new int[](2 * 10 ^^ 5 + 1);
  while (q--) {
    char t;
    int x;
    rd(t, x);
    if (t == 'L') {
      pos[x] = --l;
    } else if (t == 'R') {
      pos[x] = r++;
    } else {
      writeln(min(pos[x] - l, r - pos[x] - 1));
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

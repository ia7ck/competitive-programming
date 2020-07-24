void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int q;
  rd(q);
  while (q--) {
    long a, b, c;
    rd(a, b, c);
    long pu = a / 2 * 100, pi = (a - a / 2) * 100;
    for (auto i = max(0, b / 2 - 200); i <= min(b, b / 2 + 200); i++) {
      for (auto j = max(0, c / 2 - 200); j <= min(c, c / 2 + 200); j++) {
        if (pu + i * 10 + j == pi + (b - i) * 10 + (c - j)) {
          writeln("Yes");
          goto heaven;
        }
      }
    }
    writeln("No");
  heaven:
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

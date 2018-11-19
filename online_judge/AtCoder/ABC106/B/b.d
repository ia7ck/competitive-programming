void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  int tot = 0;
  for (int i = 1; i <= n; i += 2) {
    int k = 2;
    for (int j = 2; j * j <= i; j++) {
      if (i % j == 0)
        k += 2;
    }
    if (k == 8) {
      tot++;
    }
  }
  writeln(tot);

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

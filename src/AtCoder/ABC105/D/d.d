void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  rd(n, m);
  auto a = readln.split.to!(long[]);

  long[long] map;
  long tot = 0, mulm = 0;
  for (int i = 0; i < n; i++) {
    if (i > 0) {
      a[i] += a[i - 1];
    }
    if (a[i] % m == 0) {
      tot += (++mulm);
    } else {
      if (a[i] % m in map)
        tot += map[a[i] % m];
      if (a[i] % m in map)
        map[a[i] % m]++;
      else
        map[a[i] % m] = 1;
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

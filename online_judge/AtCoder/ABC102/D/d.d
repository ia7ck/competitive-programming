void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(long[]);
  //   3 2 4  1  2
  // 0 3 5 9 10 12
  auto acc = new long[](n + 1);
  foreach (i; 0 .. n)
    acc[i + 1] = acc[i] + a[i];

  long inf = 1_000_000_000_000_000_000;
  auto mn = inf;
  for (int i = 0, j = 2, k = 2; j <= n - 2; j++) { // (0,i] (i,j] (j,k] (k,n]
    while (i < j && acc[i] <= acc[j] - acc[i]) {
      i++;
    }
    while (k < n && acc[k] - acc[j] <= acc[n] - acc[k]) {
      k++;
    }
    for (int x = 0; x <= 1; x++) {
      for (int y = 0; y <= 1; y++) {
        auto min_val = min(acc[i - x], acc[j] - acc[i - x], acc[k - y] - acc[j], acc[n] - acc[k - y]);
        auto max_val = max(acc[i - x], acc[j] - acc[i - x], acc[k - y] - acc[j], acc[n] - acc[k - y]);
        mn = min(mn, max_val - min_val);
      }
    }
  }
  writeln(mn);
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

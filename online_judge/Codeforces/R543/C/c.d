void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n, k;
  rd(n, k);
  auto a = readln.split.to!(int[]);

  struct S {
    int cur, end, idx;
  }

  S[] q;
  foreach (i; 0 .. min(n, k)) {
    q ~= S(0, a[i], i);
  }
  alias cmp = (l, r) => (l.end - l.cur > r.end - r.cur);
  q.sort!(cmp);
  auto seen = new bool[](n);
  for (int t = 0, m = 0, i = k; m < n; t++) {
    auto r = round(100 * m, n);
    foreach (ref s; q) {
      s.cur++;
      if (s.cur == r) {
        seen[s.idx] = true;
      }
    }
    q.sort!(cmp);
    while (q.length > 0 && q[$ - 1].cur == q[$ - 1].end) {
      q.popBack;
      m++;
    }
    while (q.length < k && i < n) {
      q ~= S(0, a[i], i);
      i++;
    }
  }
  writeln(seen.count(true));
}

int round(int a, int b) {
  return (a * 2 + b) / (b * 2);
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

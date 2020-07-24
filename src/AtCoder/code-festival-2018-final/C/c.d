void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  struct T {
    long a, b;
  }

  auto f = new T[](n);
  foreach (i; 0 .. n)
    rd(f[i].a, f[i].b);
  int m;
  rd(m);
  struct Pair {
    long val;
    int idx;
  }

  auto ts = new Pair[](m);
  foreach (i; 0 .. m) {
    rd(ts[i].val);
    ts[i].idx = i;
  }

  sort!"a.a<=b.a"(f);
  sort!"a.val<=b.val"(ts);
  int j = 0;
  auto ans = new long[](m);
  fill(ans, 1000000000);
  foreach (t; ts) {
    while (j < n && f[j].a <= t.val) {
      j++;
    }
    if (j - 1 >= 0)
      ans[t.idx] = f[j - 1].b + (t.val - f[j - 1].a);
    if (j < n)
      ans[t.idx] = min(ans[t.idx], f[j].b);
  }
  writefln("%(%s\n%)", ans);

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

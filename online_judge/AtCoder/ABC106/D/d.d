void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.range;

  int n, m, q;
  rd(n, m, q);
  auto a = new int[][](n);
  foreach (_; 0 .. m) {
    int l, r;
    rd(l, r);
    a[l - 1] ~= r - 1;
  }
  foreach (i; 0 .. n)
    sort(a[i]);
  while (q--) {
    int l, r;
    rd(l, r);
    int s = 0;
    for (int i = l - 1; i < r; i++) {
      auto tri = a[i].assumeSorted.trisect(r - 1);
      s += tri[0].length + tri[1].length;
    }
    writeln(s);
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

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  rd(n, m);
  struct T {
    int a, b;
  }

  auto es = new T[](m);
  foreach (i; 0 .. m)
    rd(es[i].a, es[i].b);

  sort!"a.b<b.b"(es);
  int last = -1, cnt = 0;
  foreach (e; es) {
    if (last <= e.a) {
      last = e.b;
      cnt++;
    }
  }
  writeln(cnt);

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

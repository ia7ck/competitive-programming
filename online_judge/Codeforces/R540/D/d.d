void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  long m;
  rd(n, m);
  auto a = readln.split.to!(long[]);

  sort!"a>b"(a);
  if (m > reduce!"a+b"(a)) {
    writeln(-1);
    return;
  }
  long _s = 0;
  foreach (int i; 0 .. n) {
    _s += max(0L, a[i] - i);
  }
  if (_s >= m) {
    writeln(1);
    return;
  }
  bool enough(long k) {
    long s = 0;
    foreach (i; 0 .. n) {
      s += max(0L, a[i] - (i / k));
    }
    return s >= m;
  }

  long ng = 1, ok = n;
  while (ok - ng > 1) {
    auto md = (ok + ng) / 2;
    if (enough(md)) {
      ok = md;
    } else {
      ng = md;
    }
  }
  writeln(ok);
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

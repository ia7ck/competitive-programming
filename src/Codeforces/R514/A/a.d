void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, l, a;
  rd(n, l, a);
  auto s = new int[](n), t = new int[](n);
  foreach (i; 0 .. n)
    rd(s[i], t[i]);

  long cnt = 0;
  foreach (i; 1 .. n) {
    cnt += (s[i] - (s[i - 1] + t[i - 1])) / a;
  }
  if (n > 0) {
    cnt += (s[0]) / a;
    cnt += (l - (s[$ - 1] + t[$ - 1])) / a;
  } else {
    cnt += l / a;
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

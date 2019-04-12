void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  if (a[0] != a[$ - 1]) {
    writeln(n - 1);
    return;
  }
  int mx = 1;
  foreach (i; 1 .. n) {
    if (a[0] != a[i]) {
      mx = max(mx, i, n - i - 1);
    }
  }
  writeln(mx);

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

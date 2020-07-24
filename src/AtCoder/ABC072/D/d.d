void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n;
  rd(n);
  auto a = readln.split
    .to!(int[])
    .map!((e) => (e - 1))
    .array;

  int num = 0;
  foreach (i; 0 .. (n - 1)) {
    if (i == a[i]) {
      num++;
      swap(a[i], a[i + 1]);
    }
  }
  if (a[n - 1] == n - 1)
    num++;

  writeln(num);
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

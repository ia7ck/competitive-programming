void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(long[]);

  long s1 = 0, s2 = 0;
  foreach (i; 1 .. n) {
    if (i & 1) {
      s1 += a[i];
    } else {
      s2 += a[i];
    }
  }
  int ans = 0;
  if (s1 == s2) {
    ans++;
  }
  foreach (i; 1 .. n) {
    if (i & 1) {
      s1 -= a[i];
      s1 += a[i - 1];
    } else {
      s2 -= a[i];
      s2 += a[i - 1];
    }
    if (s1 == s2) {
      ans++;
    }
  }
  writeln(ans);
}

// 1, 2, 3, 4, 5
// ., 2, 3, 4, 5
// 1, ., 3, 4, 5
// 1, 2, ., 4, 5

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

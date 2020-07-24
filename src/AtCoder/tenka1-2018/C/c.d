void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  // a1 > a2 < a3 > a4
  // a1 - (a2)*2 + (a3)*2 - a4
  // n:偶 n/2-1個2倍+ n/2-1個2倍- 1個1倍+ 1個1倍-
  // n:奇
  //      n/2-1個2倍+ n/2個2倍- 2個1倍+ or
  //      n/2個2倍+ n/2-1個2倍- 2個1倍-

  int n;
  rd(n);
  auto a = new long[](n);
  foreach (i; 0 .. n)
    rd(a[i]);

  sort(a);
  reverse(a);
  long mx = 0;
  if (n & 1) {
    long sub1 = 0;
    foreach (i; 0 .. n) {
      if (i < n / 2 - 1) {
        sub1 += a[i] * 2;
      } else if (i < n / 2 + 1) {
        sub1 += a[i];
      } else {
        sub1 -= a[i] * 2;
      }
    }
    long sub2 = 0;
    foreach (i; 0 .. n) {
      if (i < n / 2) {
        sub2 += a[i] * 2;
      } else if (i < n / 2 + 2) {
        sub2 -= a[i];
      } else {
        sub2 -= a[i] * 2;
      }
    }
    mx = max(sub1, sub2);
  } else {
    foreach (i; 0 .. n) {
      if (i < n / 2 - 1) {
        mx += a[i] * 2;
      } else if (i == n / 2 - 1) {
        mx += a[i];
      } else if (i == n / 2) {
        mx -= a[i];
      } else {
        mx -= a[i] * 2;
      }
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

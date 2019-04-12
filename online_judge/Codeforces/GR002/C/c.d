void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  rd(n, m);
  auto a = new int[][](n, m);
  foreach (i; 0 .. n) {
    a[i] = readln.split.to!(int[]);
  }
  auto b = new int[][](n, m);
  foreach (i; 0 .. n) {
    b[i] = readln.split.to!(int[]);
  }

  if (n < 2 || m < 2) {
    bool all = true;
    foreach (i; 0 .. n) {
      foreach (j; 0 .. m) {
        all &= a[i][j] == b[i][j];
      }
    }
    writeln(all ? "Yes" : "No");
    return;
  }
  foreach (i; 0 .. (n - 1)) {
    foreach (j; 0 .. (m - 1)) {
      if (a[i][j] != b[i][j]) {
        a[i][j] ^= 1;
        a[i + 1][j] ^= 1;
        a[i][j + 1] ^= 1;
        a[i + 1][j + 1] ^= 1;
      }
    }
  }
  bool all = true;
  foreach (i; 0 .. n) {
    foreach (j; 0 .. m) {
      all &= a[i][j] == b[i][j];
    }
  }
  writeln(all ? "Yes" : "No");
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

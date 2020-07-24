void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = new char[][](n);
  foreach (i; 0 .. n) {
    a[i] = readln.chomp.to!(char[]);
  }
  if (a[0][0] == '.' || a[0][n - 1] == '.' || a[n - 1][0] == '.' || a[n - 1][n - 1] == '.') {
    writeln("NO");
    return;
  }
  for (int i = 1; i + 1 < n; i++) {
    for (int j = 1; j + 1 < n; j++) {
      if (a[i][j] == '.') {
        if (a[i - 1][j] == '#' || a[i][j - 1] == '#' || a[i + 1][j] == '#' || a[i][j + 1] == '#') {
          continue;
        }
        a[i][j] = a[i - 1][j] = a[i][j - 1] = a[i + 1][j] = a[i][j + 1] = '#';
      }
    }
  }
  foreach (i; 0 .. n) {
    foreach (j; 0 .. n) {
      if (a[i][j] == '.') {
        writeln("NO");
        return;
      }
    }
  }
  writeln("YES");
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

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n = 9;
  auto tab = new char[][](n, n);
  foreach (i; 0 .. n) {
    foreach (j; 0 .. n) {
      tab[i][j] = '.';
    }
  }

  foreach (k; 3 .. 6) {
    tab[k][k] = '#';
  }
  foreach (p; [[1, 2], [2, 0]]) {
    auto i = p[0], j = p[1];
    tab[i][j] = '#';
    tab[j][n - i - 1] = '#';
    tab[n - j - 1][i] = '#';
    tab[n - i - 1][n - j - 1] = '#';
  }

  writefln("%(%(%c%)\n%)", tab);
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

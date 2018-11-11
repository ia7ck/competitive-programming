void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  rd(n, m);
  auto c = new char[][](n, m);
  foreach (i; 0 .. n)
    c[i] = readln.chomp.to!(char[]);

  char[][] a = [['#', '#', '#'], ['#', '.', '#'], ['#', '#', '#']];
  auto b = new char[][](n, m);
  foreach (i; 0 .. n)
    foreach (j; 0 .. m)
      b[i][j] = '.';
  foreach (i; 0 .. n) {
    foreach (j; 0 .. m) {
      if (i + 3 <= n && j + 3 <= m) {
        bool ok = true;
        foreach (ii; 0 .. 3) {
          foreach (jj; 0 .. 3) {
            if (a[ii][jj] == '#') {
              ok &= c[i + ii][j + jj] == '#';
            }
          }
        }
        if (ok) {
          foreach (ii; 0 .. 3) {
            foreach (jj; 0 .. 3) {
              if (a[ii][jj] == '#') {
                b[i + ii][j + jj] = '#';
              }
            }
          }
        }
      }
    }
  }

  if (b == c) {
    writeln("YES");
  } else {
    writeln("NO");
  }
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

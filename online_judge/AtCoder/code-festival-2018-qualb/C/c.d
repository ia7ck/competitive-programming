void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);

  auto c = new char[][](n, n);
  auto _c = new char[][](n, n);
  afill(c, '.');
  afill(_c, '.');
  auto dx = [1, 0, -1, 0];
  auto dy = [0, 1, 0, -1];
  int num = 0;
  void f(int i, int j) {
    num++;
    c[i][j] = '#';
    _c[i][j] = 'X';
    foreach (k; 0 .. 4) {
      auto ni = i + dy[k], nj = j + dx[k];
      if (0 <= ni && ni < n && 0 <= nj && nj < n) {
        c[ni][nj] = '#';
      }
    }
  }

  auto ij = [[4, 1], [1, 2], [3, 3], [5, 4], [2, 5]];
  foreach (p; ij) {
    for (int i = p[0]; i < n; i += 5) {
      for (int j = p[1]; j < n; j += 5) {
        f(i, j);
      }
    }
  }
  for (int i = 2; i < n; i += 5) {
    f(i, 0);
  }
  for (int j = 4; j < n; j += 5) {
    f(0, j);
  }
  foreach (i; 0 .. n) {
    foreach (j; 0 .. n) {
      if (c[i][j] == '.') {
        f(i, j);
      }
    }
  }

  // writefln("%(%(%c%)\n%)", c);
  writeln(num);
  // writefln("%(%(%c%)\n%)", _c);
}

void afill(Range, Type)(Range r, Type value) {
  static if (is(typeof(r) == Type[])) {
    foreach (ref elem; r)
      elem = value;
  }
  else {
    foreach (ref arr; r)
      afill(arr, value);
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

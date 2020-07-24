void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.range;

  int h, w;
  rd(h, w);
  auto c = new char[][](h, w);
  iota(0, h).each!((i) => (c[i] = readln.chomp.to!(char[])));

  auto cc = new char[][](h, w);
  foreach (i; 0 .. h) {
    foreach (j; 0 .. w) {
      if (c[i][j] == '.') {
        int n = 0;
        foreach (dy; [-1, 0, 1]) {
          foreach (dx; [-1, 0, 1]) {
            if (dy == 0 && dx == 0)
              continue;
            auto ni = i + dy, nj = j + dx;
            if (ni < 0 || ni >= h || nj < 0 || nj >= w)
              continue;
            if (c[ni][nj] == '#') {
              n++;
            }
          }
        }
        cc[i][j] = (n + '0').to!(char);
      } else {
        cc[i][j] = '#';
      }
    }
  }
  writefln("%(%(%c%)\n%)", cc);
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

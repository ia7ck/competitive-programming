void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int h, w;
  rd(h, w);
  auto c = new char[][](h, w);
  foreach (i; 0 .. h)
    c[i] = readln.chomp.to!(char[]);

  char[] moves;
  bool f(int i, int pos) {
    if (i < 0) {
      moves = moves[0 .. ($ - 1)]; // ???
      return true;
    } else if (pos < 0 || pos >= w) {
      return false;
    } else if (c[i][pos] == 'x') {
      return false;
    } else {
      auto ch = ['L', 'R', 'S'], dx = [-1, 1, 0];
      foreach (j; 0 .. 3) {
        moves ~= ch[j];
        if (f(i - 1, pos + dx[j])) {
          return true;
        }
        moves = moves[0 .. ($ - 1)];
      }
      return false;
    }
  }

  int p = -1;
  foreach (j; 0 .. w) {
    if (c[h - 1][j] == 's') {
      p = j;
      break;
    }
  }
  if (f(h - 1, p)) {
    writeln(moves);
  } else {
    writeln("impossible");
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

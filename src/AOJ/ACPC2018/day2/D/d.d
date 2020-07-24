void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int r, c, si, sj, gi, gj;
  rd(r, c, si, sj, gi, gj);
  const int mod = 10 ^^ 9 + 7;
  const int inf = 10 ^^ 9;
  auto dist = new int[][](r, c), dp = new int[][](r, c);
  foreach (i; 0 .. r)
    foreach (j; 0 .. c)
      dist[i][j] = inf;
  struct T {
    int y, x;
  }

  auto dir = [T(1, 0), T(-1, 0), T(0, 1), T(0, -1)];
  auto dr = [0, r - 1], dc = [0, c - 1];
  auto q = new Queue!(T)(r * c);
  dist[si][sj] = 0;
  dp[si][sj] = 1;
  q.insertBack(T(si, sj));
  void relieve(int y, int x, int ny, int nx) {
    if (0 <= ny && ny < r && 0 <= nx && nx < c) {
      if (dist[y][x] + 1 == dist[ny][nx]) {
        (dp[ny][nx] += dp[y][x]) %= mod;
      }
      if (dist[ny][nx] == inf) {
        dist[ny][nx] = dist[y][x] + 1;
        dp[ny][nx] = dp[y][x];
        q.insertBack(T(ny, nx));
      }
    }
  }

  while (!q.empty) {
    auto cur = q.front;
    q.removeFront;
    foreach (d; dir) {
      relieve(cur.y, cur.x, cur.y + d.y, cur.x + d.x);
    }
    foreach (d; dr)
      relieve(cur.y, cur.x, d, cur.x);
    foreach (d; dc)
      relieve(cur.y, cur.x, cur.y, d);
  }
  writeln(dist[gi][gj], " ", dp[gi][gj]);
}

class Queue(T) {
private:
  int n, l = 0, r = 0;
  T[] arr;

public:
  this(int size) {
    n = size + 1;
    arr.length = n;
  }

  bool empty() {
    return l == r;
  }

  bool full() {
    return l == (r + 1) % n;
  }

  T front() {
    return arr[l];
  }

  void insertBack(T x) {
    assert(full == false);
    arr[r] = x;
    (r += 1) %= n;
  }

  void removeFront() {
    assert(empty == false);
    (l += 1) %= n;
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

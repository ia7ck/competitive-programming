void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int h, w, x;
  rd(h, w, x);
  struct T {
    int y, x;
  }

  auto q = new Queue!(T)(h * w);
  auto dist = new int[][](h, w);
  const int inf = 10 ^^ 9;
  auto c = new char[][](h, w);
  foreach (i; 0 .. h) {
    c[i] = readln.chomp.to!(char[]);
    foreach (j; 0 .. w) {
      dist[i][j] = inf;
      if (c[i][j] == '@') {
        dist[i][j] = 0;
        q.insertBack(T(i, j));
      }
    }
  }
  auto dir = [T(-1, 0), T(1, 0), T(0, -1), T(0, 1)];
  while (!q.empty) {
    auto cur = q.front;
    q.removeFront;
    foreach (d; dir) {
      auto ny = cur.y + d.y, nx = cur.x + d.x;
      if (c[ny][nx] != '#' && dist[ny][nx] == inf) {
        dist[ny][nx] = dist[cur.y][cur.x] + 1;
        q.insertBack(T(ny, nx));
      }
    }
  }

  int gy, gx;
  auto dist2 = new int[][](h, w);
  foreach (i; 0 .. h) {
    foreach (j; 0 .. w) {
      if (c[i][j] == 'S') {
        dist2[i][j] = 0;
        q.insertBack(T(i, j));
      } else {
        dist2[i][j] = inf;
        if (c[i][j] == 'G') {
          gy = i;
          gx = j;
        }
      }
    }
  }
  while (!q.empty) {
    auto cur = q.front;
    q.removeFront;
    foreach (d; dir) {
      auto ny = cur.y + d.y, nx = cur.x + d.x;
      if (c[ny][nx] != '#' && dist[ny][nx] > x && dist2[ny][nx] == inf) {
        dist2[ny][nx] = dist2[cur.y][cur.x] + 1;
        q.insertBack(T(ny, nx));
      }
    }
  }
  if (dist2[gy][gx] < inf) {
    writeln(dist2[gy][gx]);
  } else {
    writeln(-1);
  }
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

struct Edge(T) {
  int to;
  T cost;
}

T[] dijkstra(T, alias edge = Edge!(T))(edge[][] g, int s) {
  import std.algorithm : fill;
  import std.container : BinaryHeap, Array;

  const inf = T.max;
  auto dist = new T[](g.length);
  fill(dist, inf);
  dist[s] = T.init;
  alias P = edge;
  auto q = new BinaryHeap!(Array!P, "a.cost>b.cost");
  q.insert(P(s, dist[s]));
  while (q.length > 0) {
    auto cost = q.front.cost;
    auto u = q.front.to;
    q.removeFront;
    foreach (e; g[u]) {
      if (cost + e.cost < dist[e.to]) {
        dist[e.to] = cost + e.cost;
        q.insert(P(e.to, dist[e.to]));
      }
    }
  }
  return dist;
}

unittest {
  // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A
  /*
    in
    4 5 0
    0 1 1
    0 2 4
    1 2 2
    2 3 1
    1 3 5

    out
    0
    1
    3
    4
  */
  const n = 4, m = 5, s = 0;
  alias edge = Edge!(int);
  auto g = new edge[][](n);
  g[0] ~= edge(1, 1);
  g[0] ~= edge(2, 4);
  g[1] ~= edge(2, 2);
  g[2] ~= edge(3, 1);
  g[1] ~= edge(3, 5);
  auto dist = dijkstra!(int)(g, s);
  foreach (i, d; [0, 1, 3, 4]) {
    assert(dist[i] == d);
  }
}

/+
  http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3139560#1

  未検証
  https://beta.atcoder.jp/contests/abc088/submissions/2642911
  https://beta.atcoder.jp/contests/soundhound2018-summer-qual/submissions/2852092
+/

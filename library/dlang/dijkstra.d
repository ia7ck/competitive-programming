struct Edge(T){
  int to;
  T cost;
}

T[] dijkstra(T, alias edge=Edge!(T))(edge[][] g, int s){
  import std.algorithm, std.container;
  const inf=T.max;
  auto dist=new T[](g.length);
  fill(dist, inf);
  dist[s]=T.init;
  alias P=edge;
  auto q=new BinaryHeap!(Array!P, "a.cost>b.cost");
  q.insert(P(s, dist[s]));
  while(q.length>0){
    auto cost=q.front.cost;
    auto u=q.front.to;
    q.removeFront;
    foreach(e; g[u]){
      if(cost+e.cost<dist[e.to]){
        dist[e.to]=cost+e.cost;
        q.insert(P(e.to, dist[e.to]));
      }
    }
  }
  return dist;
}

/+
  http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3139560#1

  未検証
  https://beta.atcoder.jp/contests/abc088/submissions/2642911
  https://beta.atcoder.jp/contests/soundhound2018-summer-qual/submissions/2852092
+/

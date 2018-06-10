void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  alias T=int;
  const inf=1e9.to!(T);  
  struct Edge{int to; T cost;}
  int n, m; rd(n, m);
  auto g=new Edge[][](n);
  foreach(_; 0..m){
    int a, b;
    T c;
    rd(a, b, c);
    g[a]~=Edge(b, c);
    g[b]~=Edge(a, c);
  }

  /+
    a1 b1 c1
    a2 b2 c2
    .
    .
    am bm cm
  +/

  T[] dijkstra(Edge[][] g, int s){
    auto d=new T[](n);
    fill(d, inf);
    struct P{int v; T d;}
    import std.container;
    auto Q=new RedBlackTree!(P, "a.d<b.d", true);
    Q.insert(P(s, 0)); d[s]=0;
    while(Q.empty==false){
      auto cur=Q.front; Q.removeFront;
      foreach(e; g[cur.v]){
        if(d[e.to]<=d[cur.v]+e.cost) continue;
        d[e.to]=d[cur.v]+e.cost;
        Q.insert(P(e.to, d[e.to]));
      }
    }
    return d;
  }

  auto d=dijkstra(g, 0);

  /+
    d[i]=inf => s->iのパスなし
  +/
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

/+
  https://beta.atcoder.jp/contests/abc088/submissions/2642911
+/
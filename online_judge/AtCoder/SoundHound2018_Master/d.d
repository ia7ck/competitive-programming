void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  alias T=long;

  int n, m, s, t; rd(n, m, s, t);
  s--; t--;
  struct Edge{
    int to;
    T cost;
  }
  auto g_yen=new Edge[][](n);
  auto g_snu=new Edge[][](n);
  foreach(_; 0..m){
    int u, v; T a, b; rd(u, v, a, b);
    u--; v--;
    g_yen[u]~=Edge(v, a);
    g_yen[v]~=Edge(u, a);
    g_snu[u]~=Edge(v, b);
    g_snu[v]~=Edge(u, b);
  }

  const T inf=1e18.to!(long);
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
  
  auto d_yen=dijkstra(g_yen, s),
       d_snu=dijkstra(g_snu, t);
  auto ans=new long[](n);
  foreach_reverse(i; 0..n){
    ans[i]=1e15.to!(long)-d_yen[i]-d_snu[i];
    if(i+1<n) chmax(ans[i], ans[i+1]);
  }
  writefln("%(%s\n%)", ans);
}

void chmax(Type)(ref Type l, Type r){if(l<r)l=r;}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
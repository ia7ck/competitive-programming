// テスト……

class maxFlow{
  import std.typecons, std.conv, std.algorithm; // , std.stdio;
  alias T=int;
  alias Edge=Tuple!(int, "to", int, "rev", T, "cap");
  Edge[][] g;
  bool[] vis;
  auto inf=(1_000_000_000).to!(T);
 
  this(int sz){
    g.length=sz;
    vis.length=sz;
  }
 
  void addEdge(int from, int to, T cap){
    g[from]~=Edge(to, (g[to].length).to!(int), cap);
    g[to]~=Edge(from, (g[from].length-1).to!(int), (0).to!(T));
  }
 
  T flow(int i, T curf, int sink){
    if(i==sink) return curf;
    vis[i]=true;
    foreach(ref e; g[i]){
      if(vis[e.to] || e.cap==0) continue;
      auto tmpf=flow(e.to, min(curf, e.cap), sink);
      if(tmpf>0){
        e.cap-=tmpf;
        g[e.to][e.rev].cap+=tmpf;
        return tmpf;
      }
    }
    return 0;
  }
 
  T ford(int source, int sink){
    auto maxf=(0).to!(T);
    while(true){
      fill(vis, false);
      auto f=flow(source, inf, sink);
      if(f>0) maxf+=f;
      else return maxf;
    }
  }
}

/+
  https://beta.atcoder.jp/contests/soundhound2018/submissions/2072474
  https://yukicoder.me/submissions/234445
+/
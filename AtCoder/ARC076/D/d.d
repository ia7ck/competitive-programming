void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  struct P{int x, y, idx;}
  auto ps=new P[](n);
  foreach(i; 0..n){
    rd(ps[i].x, ps[i].y);
    ps[i].idx=i;
  }

  struct E{int u, v; long cost;}
  E[] edges;
  sort!((l, r)=>(l.x==r.x ? l.y<r.y : l.x<r.x))(ps);
  import std.math;
  for(int i=1; i<n; i++){
    auto d=min((ps[i-1].x-ps[i].x).abs, (ps[i-1].y-ps[i].y).abs);
    edges~=E(ps[i-1].idx, ps[i].idx, d);
  }
  sort!((l, r)=>(l.y==r.y ? l.x<r.x : l.y<r.y))(ps);
  for(int i=1; i<n; i++){
    auto d=min((ps[i-1].x-ps[i].x).abs, (ps[i-1].y-ps[i].y).abs);
    edges~=E(ps[i-1].idx, ps[i].idx, d);
  }
  sort!((l, r)=>(l.cost<r.cost))(edges);
  auto uf=new UnionFind(n);
  long tot=0;
  foreach(e; edges){
    if(uf.same(e.u, e.v)==false){
      uf.unite(e.u, e.v);
      tot+=e.cost;
    }
  }
  writeln(tot);
}

class UnionFind{
  int[] par;
  this(int n){
    par.length=n;
    foreach(i; 0..n) par[i]=i;
  }
  int find(int i){
    return par[i]=(i==par[i] ? par[i] : find(par[i]));
  }
  void unite(int i, int j){
    par[find(i)]=find(j);
  }
  bool same(int i, int j){
    return find(i)==find(j);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

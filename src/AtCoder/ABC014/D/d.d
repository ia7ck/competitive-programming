void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto tree=new tree(n);
  foreach(_; 0..(n-1)){
    int a, b; rd(a, b);
    tree.addEdge(a-1, b-1);
  }
  tree.init;
  int q; rd(q);
  while(q--){
    int a, b; rd(a, b);
    a--; b--;
    int lca=tree.getLca(a, b);
    writeln((tree.dep[a]+tree.dep[b]-tree.dep[lca]*2)+1);
  }
}

class tree{
  import std.algorithm;
  int[][] g;
  int[][] par;
  int[] dep;
  int n;
  int len=1;
  this(int n){
    this.n=n;
    g.length=n;
    while((1<<len)<=n) len++;
    par=new int[][](n, len);
    foreach(i; 0..n) fill(par[i], -1);
    dep.length=n;
  }
  void addEdge(int u, int v){
    g[u]~=v; g[v]~=u;
  }
  void init(){
    void dfs(int i=0, int j=-1){
      par[i][0]=j;
      if(j>=0) dep[i]=dep[j]+1;
      foreach(k; g[i]){
        if(j!=k) dfs(k, i);
      }
    }
    dfs();
    for(int j=0; j<len-1; j++)foreach(i; 0..n){
      if(par[i][j]>=0) par[i][j+1]=par[par[i][j]][j];
    }
  }
  int getLca(int s, int t){
    if(dep[s]<dep[t]) swap(s, t);
    foreach(i; 0..len){
      if((dep[s]-dep[t])&(1<<i)) s=par[s][i];
    }
    if(s==t) return s;
    for(auto i=len-1; i>=0; i--){
      if(par[s][i]!=par[t][i]) s=par[s][i], t=par[t][i];
    }
    return par[s][0];
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
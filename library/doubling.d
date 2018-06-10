class tree{
  import std.algorithm;
  int[][] g;
  int[][] par;
  int[] dep;
  int n;
  const int len=17; // n <= 1e5
  this(int n){
    this.n=n;
    g.length=n;
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


void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  const int n=5;
  auto t=new tree(n);
  t.addEdge(0, 1);
  t.addEdge(1, 2);
  t.addEdge(1, 3);
  t.addEdge(0, 4);
  /*
    0
    |\
    1 4
    |\
    2 3
  */
  t.init;
  assert(t.getLca(2, 3)==1);
  assert(t.getLca(2, 4)==0);
  assert(t.getLca(1, 2)==1);
  assert(t.getLca(0, 4)==0);
}

/*
  https://yukicoder.me/submissions/235784

  todo: https://beta.atcoder.jp/contests/abc014/tasks/abc014_4
*/
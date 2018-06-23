#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

struct Tree{
  vector<vector<int>> g;
  vector<vector<int>> par;
  vector<int> dep;
  int len=1;
  Tree(const vector<vector<int>> &g): g(g){
    int n=g.size();
    dep.resize(n);
    while((1<<len)<=n) len++;
    par.resize(n, vector<int>(len, -1));
  }
  void dfs(int i, int p=-1){
    par[i][0]=p;
    dep[i]=p>=0 ? dep[p]+1 : 0;
    for(int j: g[i])if(j!=p) dfs(j, i);
  }
  void init(){
    dfs(0);
    for(int j=0; j+1<len; j++)rep(i, g.size()){
      if(par[i][j]>=0) par[i][j+1]=par[par[i][j]][j];
    }
  }
  int getLca(int s, int t){
    if(dep[s]<dep[t]) swap(s, t);
    rep(i, len)if((dep[s]-dep[t])&(1<<i)) s=par[s][i];
    if(s==t) return s;
    for(int i=len-1; i>=0; i--)if(par[s][i]!=par[t][i]) s=par[s][i], t=par[t][i];
    return par[s][0];
  }
};

int main(){

  const int n=5;
  vector<vector<int>> g(n);
  g[0].push_back(1); g[1].push_back(0);
  g[1].push_back(2); g[2].push_back(1);
  g[1].push_back(3); g[3].push_back(1);
  g[0].push_back(4); g[4].push_back(0);
  Tree t(g);
  /*
      0
      |\
      1 4
      |\
      2 3
  */
  t.init();
  assert(t.getLca(2, 3)==1);
  assert(t.getLca(2, 4)==0);
  assert(t.getLca(1, 2)==1);
  assert(t.getLca(0, 4)==0);

  return 0;
}

/*
  https://beta.atcoder.jp/contests/abc014/submissions/2715346
*/
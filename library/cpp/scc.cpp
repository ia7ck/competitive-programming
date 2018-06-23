#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
#include<queue>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

struct StronglyConnectedComponents{
  vector<vector<int>> g, rev_g, h;
  vector<int> visited, order, root;
  StronglyConnectedComponents(vector<vector<int>> &g): g(g){
    int n=g.size();
    rev_g.resize(n);
    visited.resize(n, false);
    root.resize(n, -1);
    rep(i, n)for(int j: g[i]){
      rev_g[j].push_back(i);
    }
  }
  void dfs(int i){
    if(visited[i]) return;
    visited[i]=true;
    for(int j: g[i])if(visited[j]==false) dfs(j);
    order.push_back(i);
  }
  void rev_dfs(int i, int _n){
    if(root[i]>=0) return;
    root[i]=_n;
    for(int j: rev_g[i])if(root[j]<0) rev_dfs(j, _n);
  }
  void build(){
    rep(i, g.size())if(visited[i]==false) dfs(i);
    assert(order.size()==g.size());
    reverse(order.begin(), order.end());
    int _n=0;
    for(int i: order)if(root[i]<0) rev_dfs(i, _n++);
    rep(i, g.size()) assert(root[i]>=0);
    h.resize(_n);
    rep(i, g.size())for(int j: g[i]){
      int _i=root[i], _j=root[j];
      if(_i==_j) continue;
      h[_i].push_back(_j);
    }
  }
  bool same(int i, int j){ // checkしてない
    return root[i]==root[j];
  }
};

int main(){

  int n, m; cin>> n>> m;
  vector<vector<int>> g;
  rep(_, m){
    int a, b; cin>> a>> b;
    g[a].push_back(b);
  }

  StronglyConnectedComponents scc(g);
  scc.build();

  vector<int> visited(n, false);
  rep(i, n)if(visited[i]==false){
    queue<int> Q;
    visited[i]=true;
    Q.push(i);
    while(Q.size()>0){
      auto x=Q.front(); Q.pop();
      /*
        .
        .
        .
      */
      for(int y: g[x]){
        if(visited[y]) continue;
        if(scc.same(x, y)){
          visited[y]=true;
          Q.push(y);
        }
      }
    }
  }
  
  return 0;
}

/*
  http://codeforces.com/contest/999/submission/39532477
*/
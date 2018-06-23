#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
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
  void rev_dfs(int i, int itr){
    if(root[i]>=0) return;
    root[i]=itr;
    for(int j: rev_g[i])if(root[j]<0) rev_dfs(j, itr);
  }
  void compress(){
    rep(i, g.size())if(visited[i]==false) dfs(i);
    assert(order.size()==g.size());
    reverse(order.begin(), order.end());
    int itr=0;
    for(int i: order)if(root[i]<0) rev_dfs(i, itr++);
    rep(i, g.size()) assert(root[i]>=0);
  }
  void make(){
    int nn=0;
    rep(i, g.size()) nn=max(nn, root[i]+1);
    h.resize(nn);
    rep(i, g.size())for(int j: g[i]){
      int ii=root[i], jj=root[j];
      if(ii==jj) continue;
      h[ii].push_back(jj);
    }
  }
};

int main(){

  int n, m, s; cin>> n>> m>> s; s--;
  vector<vector<int>> g(n);
  rep(_, m){
    int a, b; cin>> a>> b;
    g[a-1].push_back(b-1);
  }

  StronglyConnectedComponents scc(g);
  scc.compress();
  scc.make();

  int nn=scc.h.size();
  vector<int> deg_in(nn, 0);
  rep(i, nn)for(int j: scc.h[i]) deg_in[j]++;
  int need=0;
  rep(i, nn){
    if(i==scc.root[s]) continue;
    if(deg_in[i]==0) need++;
  }
  cout<< need<< endl;

  return 0;
}
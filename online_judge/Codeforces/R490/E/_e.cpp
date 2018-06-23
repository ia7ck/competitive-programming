#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

void dfs(int i, vector<int> &visited, const vector<vector<int>> &g){
  if(visited[i]) return;
  visited[i]=true;
  for(int j: g[i]) dfs(j, visited, g);
}

int main(){

  int n, m, s; cin>> n>> m>> s; s--;
  vector<vector<int>> g(n);
  rep(_, m){
    int a, b; cin>> a>> b;
    g[a-1].push_back(b-1);
  }

  vector<vector<int>> can(n, vector<int>(n, false));
  rep(i, n) dfs(i, can[i], g);
  int _n=0;
  vector<int> root(n, -1);
  rep(i, n)if(root[i]<0){
    root[i]=_n++;
    rep(j, n)if(can[i][j] && can[j][i]) root[j]=root[i];
  }

  vector<vector<int>> h(_n);
  rep(i, n)for(int j: g[i]){
    int _i=root[i], _j=root[j];
    if(_i==_j) continue;
    h[_i].push_back(_j);
  }

  vector<int> deg_in(_n, 0);
  rep(i, _n)for(int j: h[i]) deg_in[j]++;

  int need=0;
  rep(i, _n){
    if(i==root[s]) continue;
    if(deg_in[i]==0) need++;
  }
  cout<< need<< endl;

  return 0;
}
#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
#include<queue>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  vector<vector<int>> g(n);
  rep(i, n){
    int c; cin>> c;
    rep(j, c){
      int v; cin>> v;
      g[i].push_back(v-1);
    }
  }

  vector<int> pos(n, 0);
  using P=pair<int, int>;
  struct E{int a, b;};
  vector<E> ans;
  set<P> rm;
  queue<E> q;
  rep(i, n){
    if(g[i].size()>0){
      if(g[g[i][0]].size()>0 and i==g[g[i][0]][0]){
        q.push({i, g[i][0]});
      }
    }
  }
  while(q.size()>0){
    auto e=q.front(); q.pop();
    if(e.a>e.b) swap(e.a, e.b);
    if(rm.count({e.a, e.b})) continue;
    ans.push_back(e);
    rm.insert({e.a, e.b});
    pos[e.a]++; pos[e.b]++;
    if(pos[e.a]<(int)g[e.a].size()){
      int nex=g[e.a][pos[e.a]];
      if(pos[nex]<(int)g[nex].size() and e.a==g[nex][pos[nex]]) q.push(E{e.a, nex});
    }
    if(pos[e.b]<(int)g[e.b].size()){
      int nex=g[e.b][pos[e.b]];
      if(pos[nex]<(int)g[nex].size() and e.b==g[nex][pos[nex]]) q.push(E{e.b, nex});
    }
  }

  bool ok=true;
  rep(i, n){
    for(auto j: g[i]){
      ok&=rm.count({min(i, j), max(i, j)});
    }
  }
  if(ok){
    for(auto e: ans) cout<< e.a+1<< " "<< e.b+1<< endl;
  }else{
    cout<< -1<< endl;
  }
  return 0;
}

// https://csacademy.com/submission/1709309

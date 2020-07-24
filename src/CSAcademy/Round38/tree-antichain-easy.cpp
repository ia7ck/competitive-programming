#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
#include<functional>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int t; cin>> t;
  while(t--){
    int n; cin>> n;
    vector<vector<int>> g(n);
    rep(_, n-1){
      int a, b; cin>> a>> b;
      a--; b--;
      g[a].push_back(b);
      g[b].push_back(a);
    }

    bool star=false;
    rep(i, n) star|=((int)g[i].size()==n-1);
    if(star){
      cout<< -1<< endl;
      continue;
    }
    vector<int> c(n, -1);
    function<void(int, int)> dfs=[&](int i, int color){
      c[i]=color;
      for(auto j: g[i]){
        if(c[j]<0){
          dfs(j, color^1);
        }
      }
    };
    dfs(0, 0);
    vector<int> black, white;
    rep(i, n){
      if(c[i]) black.push_back(i);
      else white.push_back(i);
    }
    set<pair<int, int>> edges;
    rep(i, n){
      for(auto j: g[i]){
        edges.insert({i, j});
        edges.insert({j, i});
      }
    }
    bool found=false;
    rep(i, black.size()){
      rep(j, 2){
        if(not edges.count({black[i], white[j]})){
          found=true;
          swap(black[i], black[black.size()-1]);
          swap(white[j], white[0]);
          goto heaven;
        }
      }
    }
  heaven:;
    assert(found);
    rep(i, black.size()) cout<< black[i]+1<< " ";
    rep(i, white.size()) cout<< white[i]+1<< (i+1<(int)white.size() ? " " : "\n");
  }

  return 0;
}

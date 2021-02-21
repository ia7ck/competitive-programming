#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  struct E{int to, c;};
  vector<vector<E>> g(n);
  rep(i, n-1){
    int a, b, c; cin>> a>> b>> c;
    g[a-1].push_back(E{b-1, c});
    g[b-1].push_back(E{a-1, c});
  }

  vector<int> sz(n, 0), ok(n, false), par(n, -1);
  function<void(int, int)> f=[&](int i, int p){
    sz[i]=1;
    par[i]=p;
    for(auto e: g[i]){
      if(e.to!=p){
        f(e.to, i);
        sz[i]+=sz[e.to];
      }else{
        ok[i]=e.c;
      }
    }
  };

  f(0, -1);
  struct T{int size, idx;};
  vector<T> sub;
  rep(i, n)if(ok[i]) sub.push_back(T{sz[i], i});
  sort(sub.begin(), sub.end(), [&](const T &l, const T &r){
    return l.size>r.size;
  });

  vector<int> vis(n, false);
  function<void(int, int)> fill=[&](int i, int p){
    vis[i]=true;
    for(auto e: g[i]){
      if(e.to!=p){
        fill(e.to, i);
      }
    }
  };
  int r=0, c=0;
  for(auto v: sub){
    if(k+r<n){
      if(vis[v.idx]==false){
        r+=v.size;
        c++;
        fill(v.idx, par[v.idx]);
      }
    }
  }
  if(k+r>=n) cout<< c<< endl;
  else cout<< -1<< endl;

  return 0;
}

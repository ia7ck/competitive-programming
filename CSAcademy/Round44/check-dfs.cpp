#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
#include<set>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, m; cin>> n>> m;
  vector<int> perm(n);
  rep(i, n){cin>> perm[i]; perm[i]--;}
  vector<vector<int>> g(n);
  rep(i, m){
    int a, b; cin>> a>> b;
    a--; b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }

  vector<int> ord(n);
  rep(i, n) ord[perm[i]]=i;
  rep(i, n){
    sort(g[i].begin(), g[i].end(), [&](const int l, const int r){
      return ord[l]<ord[r];
    });
  }
  vector<int> vis(n, false);
  vector<int> vs;
  function<void(int)> f=[&](int i){
    vs.push_back(i);
    vis[i]=true;
    for(auto j: g[i]){
      if(vis[j]==false){
        f(j);
      }
    }
  };

  f(0);
  if(perm==vs){
    cout<< 1<< endl;
  }else{
    cout<< 0<< endl;
  }
  return 0;
}

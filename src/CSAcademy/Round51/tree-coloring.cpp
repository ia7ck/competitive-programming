#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  using i64=long long;
  i64 n, k; cin>> n>> k;
  vector<vector<int>> g(n);
  rep(i, n-1){
    int a, b; cin>> a>> b;
    g[a-1].push_back(b-1);
    g[b-1].push_back(a-1);
  }

  const i64 mod=1e9+7;
  i64 tot=1;
  function<void(int, int, int)> f=[&](int i, int p, i64 cand){
    tot=tot*cand%mod;
    cand=k-1-(p>=0);
    for(auto j: g[i]){
      if(j!=p){
        f(j, i, max(0LL, cand--));
      }
    }
  };

  f(0, -1, k);
  cout<< tot<< endl;
  return 0;
}

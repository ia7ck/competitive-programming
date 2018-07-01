#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  using i64=long long;
  vector<i64> a(n);
  rep(i, n) cin>> a[i];

  vector<i64> sub(n+1, 0LL);
  rep(i, n) sub[i+1]=sub[i]+a[i];

  auto f=[&](i64 p, i64 q, i64 r, i64 s){
    i64 ret=max({p, q, r, s})-min({p, q, r, s});
    return ret;
  };

  i64 mn=1e18;
  for(int i=2; i+1<n; i++){
    i64 sl=sub[i], sr=sub[n]-sub[i];
    int j=(int)(upper_bound(sub.begin(), sub.end(), sl/2)-sub.begin());
    int k=(int)(upper_bound(sub.begin(), sub.end(), sub[i]+sr/2)-sub.begin());
    rep(t, 2)rep(u, 2) mn=min(mn, f(sub[j-t], sub[i]-sub[j-t], sub[k-u]-sub[i], sub[n]-sub[k-u]));
  }
  cout<< mn<< endl;
  return 0;
}
#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  using i64=long long;
  vector<i64> a(n);
  rep(i, n) cin>> a[i];

  if(n==1){
    cout<< 0<< endl;
    return 0;
  }
  rep(i, n) a[i]-=i;
  sort(a.begin(), a.end());
  auto cost=[&](i64 mid){
    i64 ret=0;
    rep(i, n) ret+=max(mid-a[i], a[i]-mid);
    return ret;
  };

  cout<< min(cost(a[n/2]), cost(a[n/21]));

  return 0;
}
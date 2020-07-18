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

  sort(a.begin(), a.end(), [&](const i64 l, const i64 r){return l>r;});
  i64 best=0, sum=0;
  rep(i, n){
    sum+=a[i];
    if(i&1) best=max(best, sum);
  }
  cout<< best<< endl;

  return 0;
}

// https://csacademy.com/submission/1707336/

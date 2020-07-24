#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n;
  using i64=long long;
  i64 k;
  cin>> n>> k;
  vector<i64> a(n);
  rep(i, n) cin>> a[i];

  auto f=[&](i64 h){
    i64 sum=0;
    for(auto x: a) sum+=x/h;
    return sum>=k;
  };

  i64 ok=0, ng=1e9+1;
  while(ng-ok>1){
    auto m=(ok+ng)/2;
    if(f(m)) ok=m;
    else ng=m;
  }

  cout<< ok<< endl;
  return 0;
}

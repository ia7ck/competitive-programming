#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  using i64=long long;
  auto enough=[&](i64 b){
    auto num=b;
    for(auto e: a)if(e<=b) num--;
    return num>=m;
  };
  i64 ng=0, ok=1e18;
  while(ok-ng>1){
    auto m=(ok+ng)/2;
    (enough(m) ? ok : ng) = m;
  }

  auto sum=ok*(ok+1)/2;
  for(auto e: a)if(e<ok) sum-=e;
  cout<< sum<< endl;
  return 0;
}
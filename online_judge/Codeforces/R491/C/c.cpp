#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

using i64=long long;

bool enough(i64 n, i64 k){
  i64 s1=0, s2=0;
  while(n>0){
    s1+=min(n, k);
    n=max(0LL, n-k);
    s2+=n/10;
    n=max(0LL, n-n/10);
  }
  return s1>=s2;
}

int main(){

  i64 n; cin>> n;

  i64 ng=0, ok=n;
  while(ok-ng>1){
    auto m=(ok+ng)/2;
    (enough(n, m) ? ok : ng) = m;
  }

  cout<< ok<< endl;

/*
  for(i64 k=1; k<=n; k++){
    i64 s1=0, s2=0, _n=n;
    while(_n>0){
      s1+=min(_n, k);
      _n=max(0LL, _n-k);
      s2+=_n/10;
      _n=max(0LL, _n-_n/10);
    }
    if(s1>=s2){
      cout<< k<< endl;
      return 0;
    }
  }
*/
  return 0;
}
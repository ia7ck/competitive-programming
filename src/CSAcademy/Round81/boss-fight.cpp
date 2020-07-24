#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  using i64=long long;
  i64 a1, l1, a2, l2; cin>> a1>> l1>> a2>> l2;
  int c; i64 x, y; cin>> c>> x>> y;

  for(int i=0; i<=c; i++){
    auto a=a1+x*i, l=l1+y*(c-i);
    if((l2-1)/a+1<=(l-1)/a2+1){
      cout<< 1<< endl;
      return 0;
    }
  }

  cout<< 0<< endl;
  return 0;
}
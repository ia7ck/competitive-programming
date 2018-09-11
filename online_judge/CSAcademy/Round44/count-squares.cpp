#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  using i64=long long;
  i64 n, m; cin>> n>> m;

  i64 tot=0, mod=1e9+7;
  for(i64 i=1; i<=min(n, m); i++){
    tot=(tot+i*(n-i+1)%mod*(m-i+1)%mod)%mod;
  }
  cout<< tot<< endl;

  return 0;
}

#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  using i64=long long;
  const i64 mod=1e9+7;
  vector<i64> fact(n+1), fact_inv(n+1);
  fact[0]=fact[1]=1;
  for(int i=2; i<=n; i++) fact[i]=i*fact[i-1]%mod;
  function<i64(i64, i64)> pow=[&](i64 e, i64 p){
    if(e==1) return 1LL;
    if(p==0) return 1LL;
    if(p==1) return e;
    if(p%2==0) return pow(e*e%mod, p/2);
    else return e*pow(e, p-1)%mod;
  };
  for(int i=0; i<=n; i++) fact_inv[i]=pow(fact[i], mod-2);
  vector<int> c(10);
  for(auto e: a) c[e]++;
  i64 tot=0;
  for(int i=1; i<=9; i++)if(c[i]>0){
    i64 p=1;
    rep(d, n){
      auto cmb=fact[n-1];
      for(int j=0; j<=9; j++){
        auto inv=fact_inv[c[j] - (i==j ? 1 : 0)];
        (cmb*=inv)%=mod;
      }
      if(d+1<n and c[0]>0){
        auto z=c[0]*(n-2>=0 ? fact[n-2] : 0)%mod;
        for(int j=0; j<=9; j++){
          auto inv=fact_inv[c[j] - (i==j ? 1 : 0)];
          (z*=inv)%=mod;
        }
        cmb=(cmb+mod-z)%mod;
      }
      (tot+=cmb*i%mod*p)%=mod;
      (p*=10)%=mod;
    }
  } 

  cout<< tot<< endl;
  return 0;
}
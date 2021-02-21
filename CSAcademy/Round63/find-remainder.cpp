#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
using namespace std;

#define rep(i,n) for(int i=0;i<(int)(n);i++)

long long gcd(long long a, long long b){
  if(a<b) swap(a, b);
  if(b==0) return a;
  else return gcd(b, a%b);
}

int main(){

  int n; cin>> n;
  using i64=long long;
  vector<i64> a(n), b(n);
  rep(i, n) cin>> a[i];
  rep(i, n) cin>> b[i];

  rep(i, n){
    if(a[i]<b[i]){
      cout<< -1<< endl;
      return 0;
    }
  }

  auto fac=[&](i64 k){
    vector<i64> ret;
    for(int i=1; i*i<=k; i++){
      if(k%i==0){
        ret.push_back(i);
        ret.push_back(k/i);
      }
    }
    return ret;
  };

  vector<i64> c;
  rep(i, n)if(a[i]-b[i]>0) c.push_back(a[i]-b[i]);
  if(c.size()==0){
    cout<< *max_element(a.begin(), a.end())+1<< endl;
    return 0;
  }

  i64 g;
  rep(i, c.size()){
    if(i==0) g=c[i];
    else g=gcd(g, c[i]);
  }
  auto fs=fac(g);
  sort(fs.begin(), fs.end());
  for(auto x: fs){
    rep(i, n){
      if(a[i]%x!=b[i]) goto hell;
    }
    cout<< x<< endl;
    return 0;
  hell:;
  }

  cout<< -1<< endl;
  return 0;
}

// https://csacademy.com/submission/1708058

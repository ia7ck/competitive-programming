#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  using i64=long long;
  int n; i64 k; cin>> n>> k;
  vector<i64> a(n);
  rep(i, n) cin>> a[i];

  i64 best=0;
  for(auto e: a) best+=e/2;
  if(best<k){
    cout<< -1<< endl;
    return 0;
  }

  i64 m=0, b=0;
  for(auto &e: a){
    e--;
    m+=e/2;
    b+=e/2*2;
    e%=2;
  }
  if(m>=k){
    cout<< n+k*2-1<< endl;
    return 0;
  }
  int c=0;
  for(auto e: a){
    if(e){
      if(c++, (++m)>=k){
        cout<< n+b+c<< endl;
        return 0;
      }
    }
  }

  assert(false);
  return 0;
}

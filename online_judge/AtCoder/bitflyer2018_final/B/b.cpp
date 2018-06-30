#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)


int main(){

  using i64=long long;
  int n, q; cin>> n>> q;
  vector<i64> xs(n);
  rep(i, n) cin>> xs[i];

  vector<i64> sub(n+1, 0LL);
  rep(i, n) sub[i+1]=sub[i]+xs[i];
  auto rsum=[&](int l, int r){
    return sub[r]-sub[l];
  };

  while(q--){
    i64 c, d; cin>> c>> d;
    int i=(int)(lower_bound(xs.begin(), xs.end(), c-d)-xs.begin()),
        j=(int)(lower_bound(xs.begin(), xs.end(), c)-xs.begin()),
        k=(int)(lower_bound(xs.begin(), xs.end(), c+d)-xs.begin());
    i64 ret=0;
    ret+=i*d;
    ret+=(n-k)*d;
    ret+=((j-i)*c-rsum(i, j));
    ret+=(rsum(j, k)-(k-j)*c);
    cout<< ret<< endl;
  }

  return 0;
}
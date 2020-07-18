#include<iostream>
#include<vector>
#include<algorithm>
#include<bitset>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

using i64=long long;
void chmax(i64 &l, i64 r){if(l<r)l=r;}

int main(){
  const int k=16;
  while(true){
    int n; cin>> n;
    if(n==0) break;
    vector<i64> dp(1<<k, -1);
    dp[0]=0;
    while(n--){
      int m; i64 x; cin>> m>> x;
      int mask=0;
      while(m--){
        int st, ed; cin>> st>> ed;
        st-=6; ed-=6;
        for(int i=st; i<ed; i++) mask^=(1<<i);
      }
      for(int bit=(1<<k)-1; bit>=0; bit--){
        if((bit&mask)==mask and dp[bit^mask]>=0){
          chmax(dp[bit], dp[bit^mask]+x);
        }
      }
    }
    i64 mx=0;
    rep(bit, (1<<k)) chmax(mx, dp[bit]);
    cout<< mx<< endl;
  }
  return 0;
}
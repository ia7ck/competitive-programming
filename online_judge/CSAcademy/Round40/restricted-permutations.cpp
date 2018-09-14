#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;

  vector<int> dp(n, 0);
  dp[0]=1;
  int mod=1e9+7;
  for(int i=1; i<n; i++){
    int a; cin>> a;
    vector<int> nex(n, 0);
    if(a){
      for(int j=0; j+1<n; j++){
        if(dp[j]){
          (nex[j+1]+=dp[j])%=mod;
        }
      }
      for(int j=0; j+1<=i; j++){
        (nex[j+1]+=nex[j])%=mod;
      }
    }else{
      for(int j=0; j<n; j++){
        if(dp[j]){
          (nex[j]+=dp[j])%=mod;
        }
      }
      for(int j=n-1; j>0; j--){
        (nex[j-1]+=nex[j])%=mod;
      }
    }
    dp.swap(nex);
  }

  int tot=0;
  rep(i, n) tot=(tot+dp[i])%mod;
  cout<< tot<< endl;
  return 0;
}

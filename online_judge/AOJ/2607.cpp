#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

void chmax(int &l, int r){if(l<r)l=r;}

int main(){

  int n, d, x; cin>> n>> d>> x;
  int p[d][n];
  rep(i, d)rep(j, n) cin>> p[i][j];

  const int M=100005;
  rep(i, d-1){
    // dp[j][k]:=i日目、種類jまで見て、k円持ってるときの、翌日の所持金の最大
    int dp[n+1][M];
    rep(j, n+1)rep(k, M) dp[j][k]=0;    
    for(int k=0; k<=x; k++) dp[0][k]=k; // ?
    rep(j, n){
      for(int k=0; k<=x; k++){
        chmax(dp[j+1][k], dp[j][k]);
        if(k-p[i][j]>=0){
          chmax(dp[j+1][k], dp[j+1][k-p[i][j]]+p[i+1][j]);
        }
      }
    }
    int y=0;
    for(int k=0; k<=x; k++) chmax(y, dp[n][k]);
    x=y;
  }

  cout<< x<< endl;

  return 0;
}
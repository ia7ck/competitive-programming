#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int where(char c){
  if(c=='1' or c=='4' or c=='7') return 0;
  if(c=='2' or c=='8') return 1;
  if(c=='3' or c=='6' or c=='9') return 2;
}

void chmin(int &l, int r){if(l>r)l=r;}

int main(){

  while(true){
    string s; cin>> s;
    char c=s[0];
    if(c=='#') break;

    const int inf=1e5;
    int dp[3][3][2];
    rep(i, 3)rep(j, 3)rep(k, 2) dp[i][j][k]=inf;
    for(int j=where(s[0]); j<3; j++) dp[where(s[0])][j][0]=0;
    for(int j=0; j<=where(s[0]); j++) dp[j][where(s[0])][1]=0;
    for(int i=1; i<(int)(s.size()); i++){
      int cur=where(s[i]);
      int nex[3][3][2];
      rep(_i, 3)rep(_j, 3)rep(_k, 2) nex[_i][_j][_k]=inf;
      rep(l, 3)rep(r, 3){
        if(l<=cur) chmin(nex[l][cur][1], min(dp[l][r][0], dp[l][r][1]+1));
        if(cur<=r) chmin(nex[cur][r][0], min(dp[l][r][0]+1, dp[l][r][1]));
      }
      rep(_i, 3)rep(_j, 3)rep(_k, 2) dp[_i][_j][_k]=nex[_i][_j][_k];      
    }
    int mn=inf;
    rep(i, 3)rep(j, 3)rep(k, 2) chmin(mn, dp[i][j][k]);
    cout<< mn<< endl;
  }

  return 0;
}
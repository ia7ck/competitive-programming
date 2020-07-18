#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

void chmax(int &l, int r){if(l<r)l=r;}

int main(){

  const int h=2;
  vector<string> s(h);
  cin>> s[0]>> s[1];
  int w=s[0].size();
  vector<vector<int>> a(h, vector<int>(w));
  rep(i, h)rep(j, w) a[i][j]=(s[i][j]=='0' ? 0 : 1);

  vector<int> bishwock={0b1110, 0b1011, 0b0111, 0b1101};
  vector<vector<int>> dp(w, vector<int>(1<<h, 0));
  for(int i=0; i+1<w; i++){
    rep(j, (1<<h)){
      int t=(j>>1)&1, b=(j>>0)&1;
      // if(i==1 && t==1 && b==1) cout<< "dpdpdp"<< dp[i][j]<< endl;
      if(a[0][i]>t or a[1][i]>b) continue; // unfeasible
      int _nex=(a[0][i+1]<<1)^(a[1][i+1]<<0);
      chmax(dp[i+1][_nex], dp[i][j]);
      for(int bish: bishwock){
        if(((bish>>3)&1) and t) continue;
        if(((bish>>2)&1) and b) continue;
        if(((bish>>1)&1) and (a[0][i+1])) continue;
        if(((bish>>0)&1) and (a[1][i+1])) continue;
        int nex=(bish&(1<<1))^(bish&(1<<0));
        nex=nex|(a[0][i+1]<<1)|(a[1][i+1]<<0);
        chmax(dp[i+1][nex], dp[i][j]+1);
      }
    }
    // int _mx=0;
    // rep(j, (1<<h)){
    //   if(i==0) cout<< dp[i+1][j]<< endl;
    //   chmax(_mx, dp[i+1][j]);
    // }
    // cout<< i+1<< " "<< _mx<< endl;
  }

  int mx=0;
  rep(i, (1<<h)) chmax(mx, dp[w-1][i]);
  cout<< mx<< endl;

  return 0;
}
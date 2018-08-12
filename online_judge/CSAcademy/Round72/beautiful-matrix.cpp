#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  using i64=long long;
  int n, m; cin>> n>> m;
  vector<vector<i64>> a(n, vector<i64>(m));
  rep(i, n)rep(j, m) cin>> a[i][j];

  i64 base=0;
  rep(i, n-1)rep(j, m-1){
    base+=a[i][j]+a[i+1][j]+a[i][j+1]+a[i+1][j+1];
  }
  i64 ans=base; // no swap
  i64 mxrow=-1e18;
  for(int i=1; i+1<n; i++){
    i64 sub=0;
    rep(j, m){
      if(j==0 or j==m-1){
        sub-=a[i][j];
      }else{
        sub-=a[i][j]*2;
      }
    }
    mxrow=max(mxrow, sub);
  }
  for(int i: {0, n-1}){
    i64 sub=0;
    rep(j, m){
      if(j==0 or j==m-1){
        sub+=a[i][j];
      }else{
        sub+=a[i][j]*2;
      }
    }
    ans=max(ans, base+sub+mxrow);
  }

  i64 mxcol=-1e18;
  for(int j=1; j+1<m; j++){
    i64 sub=0;
    rep(i, n){
      if(i==0 or i==n-1){
        sub-=a[i][j];
      }else{
        sub-=a[i][j]*2;
      }
    }
    mxcol=max(mxcol, sub);
  }
  for(int j: {0, m-1}){
    i64 sub=0;
    rep(i, n){
      if(i==0 or i==n-1){
        sub+=a[i][j];
      }else{
        sub+=a[i][j]*2;
      }
    }
    ans=max(ans, base+sub+mxcol);
  }

  cout<< ans<< endl;

  return 0;
}
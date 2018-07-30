#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int h, w, dh, dw; cin>> h>> w>> dh>> dw;
  vector<vector<int>> b(h+dh, vector<int>(w+dw));
  rep(i, h+dh)rep(j, w+dw) cin>> b[i][j];


  vector<vector<int>> a(h, vector<int>(w, -1));
  rep(i, dh)rep(j, w) a[i][j]=b[i][j];
  rep(i, h)rep(j, dw) a[i][j]=b[i][j];
  for(int i=dh; i<h; i++)for(int j=dw; j<w; j++){
    a[i][j]=b[i][j]-b[i-dh][j-dw];
  }

  rep(i, h){
    rep(j, w) cout<< a[i][j]<< (j+1<w ? " ": "\n");
  }
  return 0;
}
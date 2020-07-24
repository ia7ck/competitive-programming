#include<iostream>
#include<vector>
#include<algorithm>
#include<map>
#include<cmath>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  while(true){
    int r, n; cin>> r>> n;
    if(n==0 && r==0) break;

    vector<int> xl(n), xr(n), h(n);
    rep(i, n) cin>> xl[i]>> xr[i]>> h[i];
    map<int, int> mh;
    rep(i, n){
      for(int x=xl[i]; x<xr[i]; x++){
        mh[x]=max(mh[x], h[i]);
      }
    }

    double t=1e9;
    for(int x=-r+1; x<r; x++){
      t=min(t, r-sqrt(r*r-x*x)+min(mh[x-1], mh[x]));
    }

    printf("%.18f\n", t);
  }

  return 0;
}
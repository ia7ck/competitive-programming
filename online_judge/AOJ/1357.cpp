#include<iostream>
#include<vector>
#include<algorithm>
#include<cmath>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

void chmax(double &l, double r){if(l<r)l=r;}

int main(){

  int n; cin>> n;
  vector<double> r(n);
  rep(i, n) cin>> r[i];

  vector<double> x(n);
  rep(i, n){
    x[i]=r[i];
    rep(j, i){
      double a=r[j]+r[i], b=r[j]-r[i];
      chmax(x[i], x[j]+sqrt(a*a-b*b));
    }
  }

  double mx=0;
  rep(i, n) chmax(mx, x[i]+r[i]);
  printf("%.20f\n", mx);

  return 0;
}
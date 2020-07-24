#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int t, c, n; cin>> t>> c>> n;
  using i64=long long;
  vector<i64> x(n), y(n), s(n);
  rep(i, n) cin>> x[i]>> y[i]>> s[i];

  rep(i, t){
    int cnt=0;
    rep(j, n){
      if((i*c+x[j])*s[j]==y[j]*c) cnt++;
    }
    cout<< cnt<< endl;
  }

  return 0;
}

// https://csacademy.com/submission/1709121

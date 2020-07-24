#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(k), b(k);
  rep(i, k) cin>> a[i];
  rep(i, k) cin>> b[i];

  vector<int> v(n, -1);
  for(auto _a: a) v[_a-1]=1;
  for(auto _b: b) v[_b-1]=0;

  int r=0;
  rep(i, n){
    if(v[i]+v[(i+1)%n]==1){
      r++;
      v[i]=v[(i+1)%n]=-1;
    }
  }
  cout<< r+(k-r)*2<< endl;

  return 0;
}

// https://csacademy.com/submission/1709116

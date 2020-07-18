#include<iostream>
#include<vector>
#include<algorithm>
#include<cmath>
#include<functional>
using namespace std;

#define rep(i,n) for(int i=0;i<=(n);i++)

int main(){

  int n, m, k; cin>> n>> m>> k;

  function<int(int, int)> gcd=[&](int x, int y){
    if(x<y) swap(x, y);
    if(y==0) return x;
    else return gcd(y, x%y);
  };

  int tot=0;
  rep(x1, n)rep(x2, n)rep(y1, m)rep(y2, m){
    if(gcd(abs(x1-x2), abs(y1-y2))+1==k) tot++;
  }
  cout<< tot/2<< endl;

  return 0;
}
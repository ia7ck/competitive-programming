#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  int _s=0;
  for(int e: a) _s+=e;
  if((int)(1.0*_s/n+0.5)==5){
    cout<< 0<< endl;
    return 0;
  }
  sort(a.begin(), a.end());
  int cnt=1;
  rep(i, n){
    a[i]=5;
    int s=0;
    rep(j, n) s+=a[j];
    if((int)(1.0*s/n+0.5)==5){
      cout<< cnt<< endl;
      return 0;
    }
    cnt++;
  }

  assert(false);

  return 0;
}
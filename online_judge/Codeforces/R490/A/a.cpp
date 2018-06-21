#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  int c=0;
  for(int i=n-1; i>=0; i--){
    if(a[i]<=k) c++, a.pop_back();
    else break;
  }
  for(int e: a){
    if(e<=k) c++;
    else break;
  }
  cout<< c<< endl;

  return 0;
}
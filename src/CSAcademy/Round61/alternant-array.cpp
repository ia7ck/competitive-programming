#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n*2);
  rep(i, n*2) cin>> a[i];

  int o1=0, o2=0;
  rep(i, n*2){
    if(a[i]^(i&1)) o1++;
    else o2++;
  }
  cout<< min(o1/2, o2/2)<< endl;

  return 0;
}

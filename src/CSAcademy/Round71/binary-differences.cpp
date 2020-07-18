#include<iostream>
#include<vector>
#include<algorithm>
#include<numeric>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  vector<int> dp0(n), dp1(n);
  dp0[0]=(a[0]==0);
  dp1[0]=(a[0]==1);
  for(int i=1; i<n; i++){
    dp0[i]=max<int>(a[i]==0, dp0[i-1]+(a[i]==0 ? 1 : -1));
    dp1[i]=max<int>(a[i]==1, dp1[i-1]+(a[i]==1 ? 1 : -1));
  }

  cout<< *max_element(dp1.begin(), dp1.end())+*max_element(dp0.begin(), dp0.end())+1<< endl;
  return 0;
}
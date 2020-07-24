#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  int a[n][n];
  rep(i, n)rep(j, n) cin>> a[i][j];

  long long sum=0;
  rep(i, n)rep(j, i) sum+=min(a[i][j], a[j][i]);
  cout<< sum<< endl;

  return 0;
}
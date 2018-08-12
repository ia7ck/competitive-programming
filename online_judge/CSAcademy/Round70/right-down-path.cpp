#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  vector<vector<int>> a(n, vector<int>(m));
  rep(i, n)rep(j, m) cin>> a[i][j];

  int mx=0;
  rep(i, n)rep(j, m)if(a[i][j]==1){
    int l=j, d=i;
    while(0<=l and a[i][l]==1) l--;
    while(d<n and a[d][j]==1) d++;
    if(l+1<j and i+1<d) mx=max(mx, (j-l)+(d-i)-1);
  }
  cout<< mx<< endl;

  return 0;
}
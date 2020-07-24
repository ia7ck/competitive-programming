#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  int pos1=-1;
  rep(i, n)if(a[i]==1) pos1=i;
  int mn=1e9;
  for(int i=0; i+k-1<n; i++){
    int j=i+k-1;
    if(i<=pos1 and pos1<=j){
      int ret=1;
      if(k-1>1){
        if(i>0) ret+=(i-1)/(k-1)+1;
        if(j+1<n) ret+=(n-j-1-1)/(k-1)+1;
      }else{
        ret+=i;
        ret+=(n-j-1);
      }
      mn=min(mn, ret);
    }
  }

  cout<< mn<< endl;

  return 0;
}
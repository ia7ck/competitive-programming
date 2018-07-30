#include<iostream>
#include<vector>
#include<algorithm>
#include<tuple>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  int cnt=0;
  rep(_, m){
    int t; cin>> t;
    if(t==1){
      int k; cin>> k;
      k--;
      if((--a[k])<0){
        cout<< k+1<< endl;
        cnt--;
        a[k]++;
      }
    }else{
      cnt++;
    }
  }
  rep(_, cnt) cout<< 1<< endl;
  
  return 0;
}
#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  int mx=0;
  for(auto e: a) mx|=e;
  int len=n;
  vector<int> freq(32, 0);
  for(int i=0, j=0, res=0; i<n; i++, j=max(j, i)){ // [i, j)のorをmx未満に
    while(j<n and res<mx){
      rep(k, 32){
        if(a[j]&(1<<k)){
          freq[k]++;
        }
      }
      res|=a[j++];
    }

    if(res==mx){
      len=min(len, j-i);
    }
    rep(k, 32){
      if(a[i]&(1<<k)){
        if((--freq[k])==0){
          res^=(1<<k);
        }
      }
    }
  }
  cout<< max(1, len)<< endl;
  return 0;
}

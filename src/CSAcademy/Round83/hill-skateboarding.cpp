#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  double mx=0;
  for(int i=0, v=0, last=0; i<n; i++){
    if(a[i]==-1){
      v+=2;
    }else if(a[i]==0){
      if(v==0){
        mx=max<double>(mx, i-last);
        last=i+1;
        v=0;
      }else{
        v-=1;
      }
    }else{
      if(v==0){
        mx=max<double>(mx, i-last);
        last=i+1;
        v=0;
      }else if(v==1){
        mx=max<double>(mx, (double)(i-last)+0.5);
        last=i+1;
        v=0;
      }else{
        v-=2;
      }
    }
    if(i+1==n) mx=max<double>(mx, i+1-last);
  }

  cout<< mx<< endl;

  return 0;
}
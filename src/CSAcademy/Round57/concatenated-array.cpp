#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, k; cin>> n>> k;
  string s; cin>> s;
  int lz=0, tz=0;
  for(int i=0; i<n; i++){
    if(s[i]=='0') lz++;
    else break;
  }
  for(int i=n-1; i>=0; i--){
    if(s[i]=='0') tz++;
    else break;
  }
  int zz=0;
  for(int l=0, r=0; l<n; l=++r){
    while(r<n and s[r]=='0') r++;
    zz=max(zz, r-l);
  }

  if(k==1){
    cout<< max({lz, tz, zz})<< endl;
  }else{
    if(tz==n){
      cout<< (long long)tz*k<< endl;
    }else{
      cout<< max(zz, tz+lz)<< endl;
    }
  }

  return 0;
}

// https://csacademy.com/submission/1708833

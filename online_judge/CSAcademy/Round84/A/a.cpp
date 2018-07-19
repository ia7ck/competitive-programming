#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  string s; cin>> s;

  auto enough=[&](int len)->bool{
    int one=0;
    rep(i, len)if(s[i]=='1') one++;
    if(one<3) return false;
    for(int i=len; i<n; i++){
      if(s[i-len]=='1') one--;
      if(s[i]=='1') one++;
      if(one<3) return false;
    }
    return true;
  };

  int ng=2, ok=n;
  while(ok-ng>1){
    int m=(ng+ok)/2;
    if(enough(m)) ok=m;
    else ng=m; 
  }

  cout<< ok<< endl;
  return 0;
}
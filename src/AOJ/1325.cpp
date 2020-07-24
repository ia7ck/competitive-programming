#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int t; cin>> t;
  while(t--){
    int p, q; cin>> p>> q;
    int cnt=0;
    for(int m=-200; m<=200; m++)for(int n=-200; n<=200; n++){
      int sq=n*n+m*m;
      if(sq==0) continue;
      if((m*p+n*q)%sq==0 and (m*q-n*p)%sq==0){
        cnt++;
      }
    }
    if(cnt==8) cout<< "P"<< endl;
    else cout<< "C"<< endl;
  }

  return 0;
}
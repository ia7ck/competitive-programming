#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  using i64=long long;
  i64 n, x; cin>> n>> x;

  int y=0, nz=0;
  while(n>0){
    if(n%10==0){n/=10; nz++; continue;}
    y+=(10-n%10); n=n/10+1;
    if(y<=x) nz++;
    else break;
  }
  cout<< nz<< endl;

  return 0;
}
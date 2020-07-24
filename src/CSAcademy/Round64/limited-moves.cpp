#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  // vector<vector<int>> memo(100, vector<int>(100, -1));
  // function<bool(int, int)> f=[&](int n, int p){
  //   if(n==0) return 0;
  //   if(memo[n][p]>=0) return memo[n][p];
  //   int ret=true;
  //   for(int i=1; i<=min(n, p); i++) ret&=f(n-i, i);
  //   return memo[n][p]=ret^1;
  // };

  // for(int n=2; n<100; n++){
  //   cout<< n<< ": "<< f(n, n-1)<< endl;
  // }

  int n; cin>> n;
  while(n>0){
    for(int i=0; ; i++){
      if(n&(1<<i)){
        cout<< (1<<i)<< endl;
        n-=(1<<i);
        if(n==0) break;
        int ret; cin>> ret;
        n-=ret;
        break;
      }
    }
  }

  return 0;
}

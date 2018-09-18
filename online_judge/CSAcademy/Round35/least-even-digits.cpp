#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  using i64=long long;
  i64 x; cin>> x;

  auto count=[&](i64 y){
    int ret=0;
    while(y>0){
      ret+=y%2==0;
      y/=10;
    }
    return ret;
  };

  int num=count(x);
  if(num==0){
    cout<< -1<< endl;
    return 0;
  }

  auto ub=x;
  for(int i=1; i<=1000000000; i*=10){
    if(x/i%2==0){
      ub=(x/i+1)*i;
      for(int j=1; j<i; j*=10){
        ub+=1*j;
      }
      break;
    }
  }
  i64 lb=1;
  for(int i=1; i<=x; i*=10){
    for(int k=1; k<10; k++){
      if(x/i-k<0) break;
      auto y=(x/i-k)*i;
      for(int j=1; j<i; j*=10){
        y+=9*j;
      }
      if(count(y)<num){
        lb=max(lb, y);
      }
    }
  }
  cout<< (ub-x)*(x-lb)<< endl;
  return 0;
}

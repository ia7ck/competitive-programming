#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  auto q=[&](int v){
    cout<< "Q "<< v<< endl;
    int ret; cin>> ret;
    return ret;
  };

  int lb=-1, ub=1e6+1; // n!のtrailing zeroはy個以上か？ f(lb)=true, f(ub)=falseを保つ
  while(ub-lb>1){
    auto m=(lb+ub)/2;
    if(q(m)) lb=m;
    else ub=m;
  }
  int x=lb;

  auto c=[&](int v){
    int o=5, ret=0;
    while(v/o>0){
      ret+=v/o;
      o*=5;
    }
    return ret;
  };

  lb=4; ub=1e5+1; // lb: (lb)!のtrailing zeroがx未満のもののうち最大 c(lb)<xを保つ
  while(ub-lb>1){
    auto m=(lb+ub)/2;
    if(c(m)<x) lb=m;
    else ub=m;
  }

  cout<< "A "<< ub<< endl;

  return 0;
}

// https://csacademy.com/submission/1707495

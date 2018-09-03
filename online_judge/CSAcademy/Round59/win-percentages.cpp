#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int g1, p1, g2, p2;
  cin>> g1>> p1>> g2>> p2;

  auto c=[&](int num){
    return ((g1*p1+100-1)/100+num)*100/g2;
  };
  int lb=-1, ub=g2-g1+1; // lb回勝てばp2%以下、ub回だとp2%より大きい
  while(ub-lb>1){
    auto m=(lb+ub)/2;
    if(c(m)>p2) ub=m;
    else lb=m;
  }
  cout<< lb<< endl;

  return 0;
}

// https://csacademy.com/submission/1708340

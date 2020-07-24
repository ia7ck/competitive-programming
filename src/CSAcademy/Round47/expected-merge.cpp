#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
#include<map>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;

  map<int, vector<double>> memo;
  function<vector<double>(int, int)> f=[&](int l, int r){
    if(memo.count(r-l)){
      return memo[r-l];
    }
    vector<double> ret;
    if(r-l==1){
      ret.push_back(1);
    }else if(r-l==2){
      ret.push_back(3);
      ret.push_back(3);
    }else{
      auto v1=f(l, (r+l)/2);
      auto v2=f((r+l)/2, r);
      if((r-l)%2==0){
        for(auto e: v1) ret.push_back(e+(r-l));
        for(auto e: v2) ret.push_back(e+(r-l));
      }else{
        auto v11=f(l, (r+l)/2+1);
        auto v22=f((r+l)/2+1, r);
        for(auto e: v2) v1.push_back(e);
        for(auto e: v22) v11.push_back(e);
        rep(i, (r-l)){
          ret.push_back((v1[i]+v11[i])/2);
        }
        for(auto &e: ret) e+=(r-l);
      }
    }
    return memo[r-l]=ret;
  };

  auto res=f(0, n);
  rep(i, n){
    printf("%.15f", res[i]);
    if(i+1<n){
      cout<< " ";
    }else{
      cout<< endl;
    }
  }
  return 0;
}

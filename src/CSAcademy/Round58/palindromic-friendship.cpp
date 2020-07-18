#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  set<pair<int, int>> se;
  rep(_, m){
    int a, b; cin>> a>> b;
    if(a>b) swap(a, b);
    se.insert({a, b});
  }
  int mxlen=1;
  for(int i=1; i<=n; i++){
    for(int l=i-1, r=i+1, len=1; l>0 and r<=n; l--, r++){
      if(se.count({l, r})){
        len+=2;
        mxlen=max(mxlen, len);
      }else{
        break;
      }
    }
    for(int l=i, r=i+1, len=0; l>0 and r<=n; l--, r++){
      if(se.count({l, r})){
        len+=2;
        mxlen=max(mxlen, len);
      }else{
        break;
      }
    }
  }
  cout<< mxlen<< endl;

  return 0;
}

// https://csacademy.com/submission/1708793

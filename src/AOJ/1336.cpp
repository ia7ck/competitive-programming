#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  while(true){
    int n, l; cin>> n>> l;
    if(n==0 and l==0) break;
    vector<int> rtun(l+1, -1);
    vector<int> ltun(l+1, -1);
    rep(i, n){
      char dir; int pos; cin>> dir>> pos;
      if(dir=='R') rtun[pos]=i+1;
      if(dir=='L') ltun[pos]=i+1;
    }

    int rem=n, t=0;
    while(true){
      t++;
      // 右向きのアリを動かす
      for(int i=l-1; i>=1; i--){
        const auto idx=rtun[i];
        if(idx>=0){
          if(i+1==l){
            if((--rem)==0){
              cout<< t<< " "<< idx<< endl;
              goto heaven;
            }
          }else{
            rtun[i+1]=idx;
          }
          rtun[i]=-1;
        }
      }
      // 左向きのアリを動かす
      for(int i=1; i<l; i++){
        const auto idx=ltun[i];
        if(idx>=0){
          if(i==1){
            if((--rem)==0){
              cout<< t<< " "<< idx<< endl;
              goto heaven;
            }
          }else{
            ltun[i-1]=idx;
          }
          ltun[i]=-1;
        }
      }
      // 反転
      rep(i, l){
        const auto lidx=ltun[i], ridx=rtun[i];
        if(lidx>=0 and ridx>=0){
          swap(ltun[i], rtun[i]);
        }
      }
    }
    heaven:;
  }

  return 0;
}
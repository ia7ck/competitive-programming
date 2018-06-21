#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
#include<queue>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  using i64=long long;
  vector<i64> a(n);
  rep(i, n) cin>> a[i];

  vector<i64> ad[m];
  vector<i64> c(m, 0);
  for(auto e: a) c[e%m]++;
  // for(auto e: c) cout<< e<< " "; cout<< endl;
  queue<int> Q;
  rep(_i, m*2){
    // cout<< _i<< endl;
    // for(auto e: c) cout<< e<< " "; cout<< endl;    
    int i=_i%m;
    int surp=c[i]-(n/m);
    if(surp>0){
      c[i]-=surp;
      rep(_, surp) Q.push(i);
    }else if(surp<0){
      while(Q.size()>0 && c[i]<(n/m)){
        auto j=Q.front(); Q.pop();
        // cout<< _i<< " "<< j<< endl;  
        ad[j].push_back((j<i ? i-j : i+m-j));
        c[i]++;
      }
    }   
  }

  i64 sum=0;
  rep(i, n){
    auto e=a[i]%m;
    if(ad[e].size()>0){
      a[i]+=ad[e].back();
      sum+=ad[e].back();
      ad[e].pop_back();
    }
  }
  cout<< sum<< endl;
  rep(i, n) cout<< a[i]<< (i+1==n ? "\n" : " ");    

  return 0;
}

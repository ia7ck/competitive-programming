#include<iostream>
#include<vector>
#include<algorithm>
#include<queue>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, m; cin>> n>> m;
  vector<vector<int>> g(n);
  rep(i, m){
    int a, b; cin>> a>> b;
    a--; b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }


  auto answer=[&](int _a, int _b, int _c){
    cout<< _a+1<< " "<< _b+1<< " "<< _c+1<< endl;
  };

  rep(i, n){
    if(g[i].size()>=3){
      answer(g[i][0], g[i][1], g[i][2]);
      return 0;
    }
  }

  if(n==m+1){
    cout<< -1<< endl;
    return 0;
  }

  if(n%3!=0){
    cout<< -1<< endl;
    return 0;
  }

  vector<int> dist(n, 1000000000);
  dist[0]=0;
  queue<int> q;
  q.push(0);
  while(q.size()>0){
    auto i=q.front(); q.pop();
    for(auto j: g[i]){
      if(dist[j]==1000000000){
        dist[j]=dist[i]+1;
        q.push(j);
      }
    }
  }

  vector<int> ans={0};
  rep(i, n){
    if(dist[i]==n/3){
      ans.push_back(i);
    }
  }
  assert(ans.size()==3);
  answer(ans[0], ans[1], ans[2]);
  return 0;
}

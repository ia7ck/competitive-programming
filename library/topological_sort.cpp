#include<iostream>
#include<vector>
#include<algorithm>
#include<queue>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

vector<int> toposo(const vector<vector<int>> &g){
  int n=g.size();
  vector<int> deg_in(n, 0);
  rep(i, n)for(int j: g[i]) deg_in[j]++;
  queue<int> Q;
  rep(i, n)if(deg_in[i]==0) Q.push(i);
  vector<int> order;
  while(Q.size()>0){
    auto i=Q.front(); Q.pop();
    order.push_back(i);
    for(int j: g[i]){
      if((--deg_in[j])==0) Q.push(j);
    }
  }
  return order;
}

int main(){

  int n, m; cin>> n>> m;
  vector<vector<int>> g(n);
  rep(_, m){
    int a, b; cin>> a>> b;
    g[a].push_back(b);
  }

  auto order=toposo(g);
  /*
    .
    .
    .
  */

  return 0;
}

/*
  http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0519
*/
#include<iostream>
#include<vector>
#include<algorithm>
#include<queue>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

bool multi=false;
vector<int> toposo(vector<vector<int>> &g){
  int n=g.size();
  vector<int> deg_in(n, 0);
  rep(i, n)for(int j: g[i]) deg_in[j]++;
  queue<int> Q;
  rep(i, n)if(deg_in[i]==0) Q.push(i);
  vector<int> order;
  while(Q.size()>0){
    auto i=Q.front(); Q.pop();
    order.push_back(i);
    int cnt=0;
    for(int j: g[i]){
      if((--deg_in[j])==0){
        Q.push(j);
        cnt++;
      }
    }
    if(cnt>=2) multi=true;
  }
  return order;
}

int main(){

  int n, m; cin>> n>> m;
  vector<vector<int>> g(n);
  rep(_, m){
    int a, b; cin>> a>> b;
    g[a-1].push_back(b-1);
  }

  auto order=toposo(g);

  for(int i: order) cout<< i+1<< endl;
  if(multi) cout<< 1<< endl;
  else cout<< 0<< endl;

  return 0;
}
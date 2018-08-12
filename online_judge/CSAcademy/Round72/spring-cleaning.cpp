#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<vector<int>> g(n);
  rep(i, n){
    int j; cin>> j; j--;
    g[i].push_back(j);
  }

  vector<int> deg(n, 0);
  rep(i, n)for(int j: g[i]) deg[j]++;
  vector<int> visited(n, false);
  function<void(int, int)> f=[&](int i, int pre){
    if(visited[i]) return;
    visited[i]=true;
    for(int j: g[i]) f(j, i);
    if(pre>=0) cout<< pre+1<< " "<< i+1<< endl; 
  };
  rep(i, n){
    if(deg[i]==0 && visited[i]==false) f(i, -1);
  }
  rep(i, n)if(visited[i]==false) f(i, -1);

  return 0;
}
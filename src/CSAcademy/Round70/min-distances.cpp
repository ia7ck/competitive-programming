#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  struct T{int a, b, c;};
  vector<T> data(m);
  rep(i, m) cin>> data[i].a>> data[i].b>> data[i].c;

  const int inf=1e9;
  vector<vector<int>> d(n, vector<int>(n, inf));
  rep(i, n) d[i][i]=0;
  for(auto e: data){
    d[e.a-1][e.b-1]=d[e.b-1][e.a-1]=e.c;
  }
  rep(_, n)rep(i, n)rep(j, n) d[i][j]=min(d[i][j], d[i][_]+d[_][j]);
  for(auto e: data){
    if(d[e.a-1][e.b-1]!=e.c){
      cout<< -1<< endl;
      return 0;
    }
  }
  rep(i, n)rep(j, i){
    cout<< i+1<< " "<< j+1<< " "<< min(10000000, d[i][j])<< endl;
  }

  return 0;
}
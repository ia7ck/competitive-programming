#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int h, w; cin>> h>> w;
  vector<vector<char>> a(h, vector<char>(w));
  rep(i, h)rep(j, w) cin>> a[i][j];

  struct P{int i, j;};
  auto mv=[&](int i, int j){
    if(a[i][j]=='U') i--;
    else if(a[i][j]=='D') i++;
    else if(a[i][j]=='L') j--;
    else if(a[i][j]=='R') j++;
    return P{i, j};
  };

  vector<vector<int>> vis(h, vector<int>(w, false));
  vector<vector<int>> memo(h, vector<int>(w, -1));
  function<bool(int, int)> f=[&](int i, int j)->bool{
    if(i<0 or i>=h or j<0 or j>=w) return true;
    if(memo[i][j]>=0) return memo[i][j];
    if(vis[i][j]) return false;
    vis[i][j]=true;
    auto nx=mv(i, j);
    return memo[i][j]=f(nx.i, nx.j);
  };

  int tot=0;
  rep(i, h)rep(j, w){
    if(f(i, j)) tot++;
  }
  cout<< tot<< endl;

  return 0;
}
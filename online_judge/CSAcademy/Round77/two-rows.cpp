#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int m; cin>> m;
  vector<vector<int>> a(2, (vector<int>(m)));
  rep(i, 2)rep(j, m) cin>> a[i][j];

  using i64=long long;
  const i64 nil=-1e18;
  vector<vector<vector<vector<i64>>>>
    memo(2, vector<vector<vector<i64>>>
      (m, vector<vector<i64>>
        (2, vector<i64>
          (3, nil))));
  // d: 0:上から 1:下から 2:左から 来た
  function<i64(int, int, int, int)> f=[&](int i, int j, int t, int d)->i64{
    if(i==1 and j==m-1) return a[i][j];
    if(memo[i][j][t][d]!=nil) return memo[i][j][t][d];
    i64 ret;
    if(t==1){
      ret=1e17;
      if(i-1>=0 and d!=0) ret=min(ret, f(i-1, j, t^1, 1));
      if(i+1<2 and d!=1) ret=min(ret, f(i+1, j, t^1, 0));
      if(j+1<m) ret=min(ret, f(i, j+1, t^1, 2));
    }else{
      ret=-1e17;
      if(i-1>=0 and d!=0) ret=max(ret, f(i-1, j, t^1, 1));
      if(i+1<2 and d!=1) ret=max(ret, f(i+1, j, t^1, 0));
      if(j+1<m) ret=max(ret, f(i, j+1, t^1, 2));
    }
    return memo[i][j][t][d]=ret+a[i][j];
  };

  cout<< f(0, 0, 1, 2)<< endl;

  return 0;
}
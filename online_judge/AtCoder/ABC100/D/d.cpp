#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  using i64=long long;
  vector<i64> x(n), y(n), z(n);
  rep(i, n) cin>> x[i]>> y[i]>> z[i];

  i64 mx=0;
  rep(bit, (1<<3)){
    int o1=(bit&1) ? 1 : -1,
        o2=(bit&2) ? 1 : -1,
        o3=(bit&4) ? 1 : -1;
    vector<i64> arr;
    rep(i, n) arr.push_back(x[i]*o1+y[i]*o2+z[i]*o3);
    sort(arr.rbegin(), arr.rend());
    i64 sum=0;
    rep(i, m) sum+=arr[i];
    mx=max(mx, sum);
  }

  cout<< mx<< endl;

  return 0;
}
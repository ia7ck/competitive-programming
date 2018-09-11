#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  using i64=long long;
  i64 d, l; int n; cin>> d>> l>> n;

  vector<i64> a(n+1, 0);
  rep(i, n) cin>> a[i+1];
  a.push_back(d);
  sort(a.begin(), a.end());
  long double s=0;
  int cnt=0;
  for(int i=1; i<=n+1; i++){
    auto r=max(0LL, a[i]-a[i-1]-l);
    s+=r*r/4.0;
    cnt++;
  }
  s/=d;
  printf("%.18Lf\n", s);

  return 0;
}

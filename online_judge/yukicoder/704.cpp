#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

struct SegmentTree{
  int n=1, inf=1e8;
  vector<int> seg;
  SegmentTree(int m){
    while(n<m) n*=2;
    seg.resize(n*2-1, inf);
  }
  void update(int i, int x){
    i+=n-1;
    seg[i]=x;
    while(i>0){
      i=(i-1)/2;
      seg[i]=min(seg[i*2+1], seg[i*2+2]);
    }
  }
  int getmin(int ql, int qr){
    return getmin(ql, qr, 0, 0, n);
  }
  int getmin(int ql, int qr, int i, int il, int ir){
    if(qr<=il || ir<=ql) return inf;
    if(ql<=il && ir<=qr) return seg[i];
    int m=(ir+il)/2;
    int vl=getmin(ql, qr, i*2+1, il, m),
        vr=getmin(ql, qr, i*2+2, m, ir);
    return min(vl, vr);
  }
};

void chmin(int &l, int r){if(l>r)l=r;}

int main(){

  int n; cin>> n;
  vector<int> a(n+1);
  rep(i, n) cin>> a[i+1];
  vector<int> x(n+1);
  rep(i, n) cin>> x[i+1];
  vector<int> y(n+1);
  rep(i, n) cin>> y[i+1];

  const int inf=1e8;
  SegmentTree seg_l(n), seg_r(n);

  vector<int> dp(n+1, inf);
  dp[0]=0;
  for(int i=1, j=1; i<=n; i++){
    seg_r.update(i-1, x[i]+y[i]+dp[i-1]);
    while(j<=i && x[j]<=a[i]){
      seg_l.update(j-1, -x[j]+y[j]+dp[j-1]);
      seg_r.update(j-1, inf);
      j++;
    }
    dp[i]=dp[i-1]+abs(a[i]-x[i])+y[i];
    chmin(dp[i], seg_l.getmin(0, i)+a[i]);
    chmin(dp[i], seg_r.getmin(0, i)-a[i]);
  }

  cout<< dp[n]<< endl;

  return 0;
}
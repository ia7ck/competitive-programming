#include<iostream>
#include<vector>
#include<algorithm>
#include<set>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

struct SegmentTree{
  using i64=long long;
  int n=1;
  vector<i64> dat;
  SegmentTree(int N){
    while(n<N) n*=2;
    dat.resize(n*2-1, 0LL);
  }
  void add(int i, i64 x){
    dat[i+=n-1]+=x;
    while(i>0){
      i=(i-1)/2;
      dat[i]=dat[i*2+1]+dat[i*2+2];
    }
  }
  i64 sum(int l, int r){
    return sum(l, r, 0, 0, n);
  }
  i64 sum(int ql, int qr, int i, int il, int ir){
    if(qr<=il or ir<=ql) return 0LL;
    if(ql<=il and ir<=qr) return dat[i];
    auto m=(il+ir)/2;
    return sum(ql, qr, i*2+1, il, m)+sum(ql, qr, i*2+2, m, ir);
  }
};

int main(){

  int n; cin>> n;
  struct P{int x, y;};
  vector<P> points;
  int m=0;
  rep(i, n){
    int x, y; cin>> x>> y; m=max(m, y);
    points.push_back(P{x-1, y-1});
  }
  sort(points.begin(), points.end(), [&](const P &l, const P &r){
    return l.x==r.x ? l.y<r.y : l.x<r.x;
  });

  SegmentTree t(m);
  using i64=long long;
  i64 tot=0, mod=1e9+7;
  for(auto p: points){
    auto ad=(t.sum(p.y+1, m)+1)%mod;
    tot=(tot+ad)%mod;
    t.add(p.y, ad);
  }
  cout<< tot<< endl;

  return 0;
}

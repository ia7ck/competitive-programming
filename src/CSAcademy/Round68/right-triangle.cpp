#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

template<typename T>
struct SegmentTree{
  int n=1;
  vector<T> seg;
  SegmentTree(int m){
    while(n<m) n*=2;
    seg.resize(n*2-1, 0);
  }
  void update(int i, T x){
    i+=n-1;
    seg[i]=x;
    while(i>0){
      i=(i-1)/2;
      seg[i]=seg[i*2+1]+seg[i*2+2];
    }
  }
  T sum(int ql, int qr){
    return sum(ql, qr, 0, 0, n);
  }
  T sum(int ql, int qr, int i, int il, int ir){
    if(qr<=il || ir<=ql) return 0;
    if(ql<=il && ir<=qr) return seg[i];
    int m=(ir+il)/2;
    auto vl=sum(ql, qr, i*2+1, il, m),
      vr=sum(ql, qr, i*2+2, m, ir);
    return vl+vr;
  }
};

int main(){

  int n; cin>> n;
  struct P{
    long long x, y;
    int i;
  };
  long long mm=0;
  vector<P> co(n);
  rep(i, n){
    co[i].i=i;
    cin>> co[i].x>> co[i].y;
    mm=max(mm, co[i].x);
  }
  sort(co.begin(), co.end(), [&](const P &l, const P &r){
    return l.y*r.x<r.y*l.x;
  });
  SegmentTree<int> t((int)mm+1);
  vector<int> ans(n);
  for(auto p: co){
    ans[p.i]=t.sum(0, p.x);
    t.update(p.x, 1);
  }
  for(auto a: ans) cout<< a<< endl;

  return 0;
}


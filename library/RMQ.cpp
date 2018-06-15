#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;

struct SegmentTree{
  int n=1, inf=1e9;
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

int main(){

  int n; cin>> n;
  SegmentTree seg(n);
  // ...

  return 0;
}
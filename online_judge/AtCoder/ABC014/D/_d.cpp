#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

struct T{
  int val, idx;
  T(int v, int i): val(v), idx(i){};
};

struct SegmentTree{
  int n=1;
  const int inf=1e9;
  const T nil=T(inf, -1);
  vector<T> dat;
  SegmentTree(){}
  SegmentTree(const int nn){resize(nn);}
  void resize(const int nn){
    while(n<=nn) n*=2;
    dat.resize(n*2-1, nil); 
  }
  T _min(const T &l, const T &r){
    if(l.val==r.val){
      if(l.idx<r.idx) return l;
      else return r;
    }else{
      if(l.val<r.val) return l;
      else return r;
    }
  }
  void update(int i, T x){
    i+=n-1;
    dat[i]=x;
    while(i>0){
      i=(i-1)/2;
      dat[i]=_min(dat[i*2+1], dat[i*2+2]);
    }
  }
  T getmin(int ql, int qr, int i, int il, int ir){
    if(qr<=il || ir<=ql) return nil;
    if(ql<=il && ir<=qr) return dat[i];
    int m=(il+ir)/2;
    T res_l=getmin(ql, qr, i*2+1, il, m),
      res_r=getmin(ql, qr, i*2+2, m, ir);
    return _min(res_l, res_r);
  }
  T getmin(int ql, int qr){
    return getmin(ql, qr, 0, 0, n);
  }
};

struct Tree{
  vector<vector<int>> g;
  vector<int> euler, pos, dep;
  SegmentTree seg;
  Tree(const vector<vector<int>> &g): g(g){
    int n=g.size();
    pos.resize(n, -1);
    dep.resize(n);
  }
  void dfs(int i, int p=-1){
    euler.push_back(i);
    dep[i]=p>=0 ? dep[p]+1 : 0;
    for(int j: g[i])if(j!=p){
      dfs(j, i);
      euler.push_back(i);
    }
  }
  void euler_tour(){
    dfs(0);
    rep(i, (int)(euler.size()))if(pos[euler[i]]<0) pos[euler[i]]=i;
  }
  void seg_init(){
    vector<T> data;
    for(int i: euler) data.push_back(T(dep[i], i));
    seg.resize(data.size());
    rep(i, (int)(data.size())) seg.update(i, data[i]);    
  }
  int get_lca(int a, int b){
    int l=pos[a], r=pos[b];
    if(l>r) swap(l, r);
    return seg.getmin(l, r+1).idx;
  }
  int dist(int a, int b){
    int lca=get_lca(a, b);
    assert(lca>=0);
    return dep[a]+dep[b]-dep[lca]*2;
  }
};

int main(){

  int n; cin>> n;
  vector<vector<int>> g(n);
  rep(_, n-1){
    int a, b; cin>> a>> b;
    a--; b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }

  Tree tree(g);
  tree.euler_tour();
  tree.seg_init();

  int q; cin>> q;
  while(q--){
    int a, b; cin>> a>> b;
    a--; b--;
    cout<< tree.dist(a, b)+1<< endl;
  }

  return 0;
}
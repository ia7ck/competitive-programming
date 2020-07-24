#include<iostream>
#include<vector>
#include<algorithm>
#include<map>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

struct SegmentTree{
  int n=1;
  vector<int> dat;
  SegmentTree(int N){
    while(n<N) n*=2;
    dat.resize(n*2-1, 0);
  }
  void add(int i, int x){
    dat[i+=n-1]+=x;
    while(i>0){
      i=(i-1)/2;
      dat[i]=dat[i*2+1]+dat[i*2+2];
    }
  }
  int sum(int ql, int qr){
    return sum(ql, qr, 0, 0, n);
  }
  int sum(int ql, int qr, int i, int il, int ir){
    if(qr<=il or ir<=ql) return 0;
    if(ql<=il and ir<=qr) return dat[i];
    auto m=(ir+il)/2;
    return sum(ql, qr, i*2+1, il, m)+sum(ql, qr, i*2+2, m, ir);
  }
};

int main(){

  using i64=long long;
  int n; i64 t; cin>> n>> t;
  vector<i64> a(n);
  rep(i, n) cin>> a[i];

  vector<i64> sum(n+1, 0);
  rep(i, n) sum[i+1]=sum[i]+a[i];
  map<i64, i64> ord;
  vector<i64> v=sum;
  sort(v.begin(), v.end());
  v.erase(unique(v.begin(), v.end()), v.end());
  rep(i, v.size()) ord[v[i]]=i;
  i64 tot=0;
  SegmentTree tree(v.size());
  for(int i=0; i<=n; i++){
    auto j=upper_bound(v.begin(), v.end(), sum[i]-t)-v.begin();
    tot+=tree.sum(j, v.size());
    tree.add(ord[sum[i]], 1);
  }
  cout<< tot<< endl;

  return 0;
}

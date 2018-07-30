#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

struct SquareRootDecomposition{
  int D=1;
  vector<int> buc, val;
  SquareRootDecomposition(int n){
    while(D*D<n) D++;
    buc.resize(D, 0);
    val.resize(D*D, 0);
    rep(i, n) add(i, 1);
  }
  void add(int i, int x){
    val[i]+=x;
    buc[i/D]+=x;
  }
  int sum(int l, int r){ // [l, r)
    int ret=0;
    rep(k, D){
      if((k+1)*D<=l or r<=k*D) continue;
      if(l<=k*D and (k+1)*D<=r) ret+=buc[k];
      else for(int i=max(l, k*D); i<min(r, (k+1)*D); i++) ret+=val[i];
    }
    return ret;
  }
  int lower_bound(int sum){ // sum[0, r)がsum以下の最大のr
    int s=0;
    rep(k, D){
      if((s+=buc[k])>sum){
        s-=buc[k];
        for(int i=k*D; i<(k+1)*D; i++){
          if((s+=val[i])>sum) return i;
        }
      }
    }
    return -1;
  }
};

int main(){

  using i64=long long;
  int n; i64 k; cin>> n>> k;

  vector<int> perm;
  SquareRootDecomposition t(n);

  for(int i=1; i<=n; i++){
    auto w=(i64)(n-i)*(n-i-1)/2;
    auto j=t.lower_bound(max(0LL, k-w));
    perm.push_back(j+1);
    k-=t.sum(0, j);
    t.add(j, -1);
  }

  rep(i, n) cout<< perm[i]<< (i+1<n ? " " : "\n");
  return 0;
}

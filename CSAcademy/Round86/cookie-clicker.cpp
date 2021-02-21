#include<iostream>
#include<vector>
#include<algorithm>
#include<numeric>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, c, s; cin>> n>> c>> s;
  vector<int> a(n), b(n);
  rep(i, n) cin>> a[i]>> b[i];
  vector<int> perm(n);
  iota(perm.begin(), perm.end(), 0);
  int tm=1e9;
  do{
    rep(bit, 1<<n){
      vector<int> idx;
      rep(i, n)if(bit&(1<<i)) idx.push_back(perm[i]);
      int d=0, t=0, _s=s;
      for(int j: idx){
        if(a[j]<=d){
          d-=a[j];
          _s+=b[j];
        }else{
          int x=(a[j]-d+_s-1)/_s, y=(c-d+_s-1)/_s;
          if(y<=x){
            tm=min(tm, t+y);
            goto heaven;
          }else{
            t+=x;
            d=d+_s*x-a[j];
            _s+=b[j];
          }
        }
      }
      assert(d<c);
      tm=min(tm, t+(c-d+_s-1)/_s);
    heaven:;
    }
  }while(next_permutation(perm.begin(), perm.end()));
  cout<< tm<< endl;

  return 0;
}

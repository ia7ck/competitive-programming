#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

long long gcd(long long a, long long b){
  if(a<b) swap(a, b);
  if(b==0LL) return a;
  return gcd(b, a%b);
}

long long lcm(long long a, long long b){
  return a/gcd(a, b)*b;
}

int main(){

  int n, m; cin>> n>> m;
  using i64=long long;
  vector<vector<i64>> a(n, vector<i64>(m));
  rep(i, n){
    rep(j, m) cin>> a[i][j];
  }

  vector<vector<i64>> v(n), u(m);
  rep(i, n){
    rep(j, m){
      v[i].push_back(a[i][j]);
      u[j].push_back(a[i][j]);
    }
  }

  vector<i64> vv, uu;
  for(auto _v: v){
    i64 l=1LL;
    for(auto e: _v){
      l=lcm(l, e);
      if(l>1000000000){
        cout<< -1<< endl;
        return 0;
      }
    }
    vv.push_back(l);
  }
  for(auto _u: u){
    i64 l=1LL;
    for(auto e: _u){
      l=lcm(l, e);
      if(l>1000000000){
        cout<< -1<< endl;
        return 0;
      }
    }
    uu.push_back(l);
  }

  rep(i, n){
    rep(j, m){
      if(a[i][j]!=gcd(vv[i], uu[j])){
        cout<< -1<< endl;
        return 0;
      }
    }
  }

  rep(i, n) cout<< vv[i]<< (i+1<n ? " " : "\n");
  rep(i, m) cout<< uu[i]<< (i+1<m ? " " : "\n");

  return 0;
}

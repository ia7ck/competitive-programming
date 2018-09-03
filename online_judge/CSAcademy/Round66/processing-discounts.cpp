#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; long x; cin>> n>> x;
  struct T{long a, b;};
  vector<T> w(n);
  rep(i, n) cin>> w[i].a>> w[i].b;

  sort(w.begin(), w.end(), [&](const T &l, const T &r){return l.a<r.a;});
  long minus=0, best=x;
  for(auto e: w){
    minus+=e.b;
    best=min(best, max(x, e.a)-minus);
  }
  cout<< best<< endl;

  return 0;
}

// https://csacademy.com/submission/1707355

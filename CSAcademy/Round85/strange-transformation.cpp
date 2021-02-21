#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, a, b; cin>> n>> a>> b;
  vector<int> xs(n);
  rep(i, n) cin>> xs[i];

  auto ng=[](){cout<< "-1"<< endl; exit(0);};

  struct P{
    int p2, p3;
    int r;
    P(){}
    P(int _p2, int _p3, bool _r):p2(_p2), p3(_p3), r(_r){}
  };
  auto f=[](int y){
    P ret(0, 0, 0);
    while(y%2==0) y/=2, ret.p2++;
    while(y%3==0) y/=3, ret.p3++;
    ret.r=y;
    return ret;
  };

  auto ap=f(a), bp=f(b);
  if(ap.r!=bp.r) ng();
  if(ap.p2<bp.p2 or ap.p3>bp.p3) ng();

  vector<P> xp(n);
  rep(i, n){
    xp[i]=f(xs[i]);
    if(xp[i].r!=ap.r) ng();
    if(xp[i].p2>ap.p2 or xp[i].p3>bp.p3) ng();
  }
  sort(xp.begin(), xp.end(), [](const P &l, const P &r){
    if(l.p2==r.p2) return l.p3<r.p3;
    else return l.p2>r.p2;
  });
  rep(i, n-1){
    if(xp[i].p3>xp[i+1].p3) ng();
  }
  P p=ap; int v=a;
  xp.push_back(bp);
  for(auto x: xp){
    while(p.p2>x.p2){
      cout<< v<< " ";
      p.p2--; v/=2;
    }
    while(p.p3<x.p3){
      cout<< v<< " ";
      p.p3++; v*=3;
    }
  }
  cout<< b << endl;
  return 0;
}
#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int f(const vector<int> &b, const vector<int> &a){
  int n=b.size();
  {
    int s1=0, s2=0;
    rep(i, n) s1+=b[i], s2+=a[i];
    if(s1!=s2) return 10000000;
  }
  vector<int> c=b;
  int s=0;
  rep(i, n)if(c[i]!=a[i]){
    int j=i;
    while(c[j]!=a[i]) j++;
    s+=(j-i);
    for(int k=j; k>i; k--) swap(c[k-1], c[k]);
  }
  return s;
}

int main(){

  int n, m; cin>> n>> m;
  vector<int> b(n);
  rep(i, n) cin>> b[i];
  vector<int> p(m);
  rep(i, m) cin>> p[i];

  vector<int> a0, a1;
  rep(i, m){
    rep(_, p[i]){
      a0.push_back(i%2);
      a1.push_back((i%2)^1);
    }
  }

  cout<< min(f(b, a0), f(b, a1))<< endl;

  return 0;
}
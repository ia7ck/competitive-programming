#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, m; cin>> n>> m;
  vector<vector<int>> in(n), out(n);
  rep(i, m){
    int l, r; cin>> l>> r;
    in[l-1].push_back(i);
    out[r-1].push_back(i);
  }

  set<int> se;
  int base=0;
  vector<int> v(m, 0);
  rep(i, n){
    for(auto j: in[i]) se.insert(j);
    if(se.size()==1) v[*(se.begin())]++;
    if(se.size()==0) base++;
    for(auto j: out[i]) se.erase(j);
  }
  for(auto x: v) cout<< x+base<< endl;

  return 0;
}

// https://csacademy.com/submission/1708141

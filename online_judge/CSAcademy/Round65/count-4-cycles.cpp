#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  set<pair<int, int>> s;
  rep(i, n-1){
    int a, b; cin>> a>> b;
    a--; b--;
    if(a>b) swap(a, b);
    s.insert({a, b});
  }
  int cnt=0;
  rep(i, n-1){
    int a, b; cin>> a>> b;
    a--; b--;
    if(a>b) swap(a, b);
    if(s.count({a, b})) cnt++;
  }
  cout<< cnt<< endl;

  return 0;
}

// https://csacademy.com/submission/1707405

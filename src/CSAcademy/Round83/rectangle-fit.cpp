#include<iostream>
#include<vector>
#include<algorithm>
#include<queue>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, s; cin>> n>> s;
  struct P{int x, y;};
  vector<P> coords;
  rep(i, n){
    int x, y; cin>> x>> y;
    coords.push_back(P{x, y});
  }

  auto solve=[](int n, int s, vector<P> &coords){
    sort(coords.begin(), coords.end(), [&](const P &l, const P &r){
      return l.x==r.x ? l.y<r.y : l.x<r.x;
    });
    priority_queue<int> height;
    int mx=0;
    for(int i=0, w=1; w*w<=s; w++){
      while(i<n and coords[i].x<=w){
        height.emplace(coords[i++].y);
      }
      int h=s/w;
      while(height.size()>0 and height.top()>h) height.pop();
      mx=max<int>(mx, height.size());
    }
    return mx;
  };

  int mx=0;
  mx=max(mx, solve(n, s, coords));
  for(auto &p: coords) swap(p.x, p.y);
  mx=max(mx, solve(n, s, coords));
  cout<< mx<< endl;
  return 0;
}
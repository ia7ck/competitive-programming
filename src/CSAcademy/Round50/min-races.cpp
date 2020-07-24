#include<iostream>
#include<vector>
#include<algorithm>
#include<set>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  vector<pair<int, int>> a;
  rep(i, n){
    int x, y; cin>> x>> y;
    a.push_back({x, -y});
  }

  sort(a.begin(), a.end());
  set<int> s;
  for(auto b: a){
    if(s.size()==0){
      s.insert(-b.second);
    }else{
      auto itr=s.upper_bound(-b.second);
      if(itr!=s.begin()){
        s.erase(prev(itr));
      }
      s.insert(-b.second);
    }
  }
  cout<< s.size()<< endl;

  return 0;
}

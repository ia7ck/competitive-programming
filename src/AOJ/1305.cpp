#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
#include<map>
#include<unordered_map>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

unordered_map<string, set<string>> memo;
set<string> f(map<string, string> &mp, const string &g){
  if(memo.count(g)) return memo[g];
  set<string> ret;
  if(mp.count(g)==0){
    ret.insert(g);
  }else{
    string ms=mp[g];
    string member="";
    rep(i, (int)(ms.size())){
      if(ms[i]==',' || ms[i]=='.'){
        auto retret=f(mp, member);
        for(auto e: retret) ret.insert(e);
        member="";
      }else{
        member+=ms[i];
      }
    }
  }
  return memo[g]=ret;
}

int main(){

  int n;
  while(true){
    cin>> n;
    if(n==0) break;
    memo.clear();
    map<string, string> mp;
    vector<string> gs;
    string root="";
    rep(i, n){
      string s; cin>> s;
      int _i=(int)s.find(":");
      string group=s.substr(0, _i);
      if(i==0) root=group;
      gs.push_back(group);
      mp[group]=s.substr(_i+1);
    }
    auto ret=f(mp, root);
    cout<< ret.size()<< endl;
    // for(auto s: ret) cout<< s<< endl;
  }

  return 0;
}
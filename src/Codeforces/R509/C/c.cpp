#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
#include<map>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, m, d; cin>> n>> m>> d;
  vector<int> _a(n);
  rep(i, n) cin>> _a[i];
  map<int, int> ord;
  rep(i, n) ord[_a[i]]=i;

  vector<int> a=_a;
  sort(a.begin(), a.end());
  vector<int> used(n, false);
  set<pair<int, int>> s;
  rep(i, n) s.insert({a[i], i});
  int need=0;
  vector<int> ans(n, -1);
  rep(i, n){
    if(used[i]) continue;
    used[i]=true;
    need++;
    auto cur=a[i];
    ans[ord[cur]]=need;
    while(cur<=1000000000){
      auto itr=s.lower_bound({cur+d+1, -1}); // ?
      if(itr==s.end()) break;
      // cout<< (*itr).first << " "<< (*itr).second<< endl;
      used[(*itr).second]=true;
      cur=(*itr).first;
      ans[ord[cur]]=need;
      s.erase(itr);
    }
  }
  cout<< need<< endl;
  rep(i, n) cout<< ans[i]<< (i+1<n ? " " : "\n");

  return 0;
}

#include<iostream>
#include<vector>
#include<algorithm>
#include<map>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  map<char, int> mp;
  vector<string> strs(n);
  rep(i, n){
    cin>> strs[i];
    mp[strs[i][0]]++;
  }

  int cnt=0;
  for(auto s: strs){
    if((int)s.size()<=n-1){
      if(mp[s[0]]-->=2){
        mp[s[0]]--;
        bool ok=true;
        for(int i=1; i<(int)s.size(); i++){
          ok&=(mp[s[i]]-->=1);
        }
        if(ok){
          cnt++;
        }
        for(int i=1; i<(int)s.size(); i++) mp[s[i]]++;
        mp[s[0]]++;
      }
      mp[s[0]]++;
    }
  }
  cout<< cnt<< endl;

  return 0;
}

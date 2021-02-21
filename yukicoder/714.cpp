#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
#include<map>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int q; cin>> q;
  map<string, int> mp;
  vector<map<int, int>> a(30);
  while(q--){
    int t; cin>> t;
    if(t==0){
      int n, m; cin>> n>> m;
      rep(_, m){
        string s; cin>> s;
        if(mp.count(s)==0) mp[s]=(int)mp.size();
        a[n][mp[s]]++;
      }
    }else if(t==1){
      string neta; cin>> neta;
      if(mp.count(neta)==0){
        cout<< -1<< endl;
        continue;
      }
      int id=mp[neta];
      for(int i=1; i<=20; i++){
        if(a[i].count(id)){
          cout<< i<< endl;
          if((--a[i][id])==0) a[i].erase(id);
          goto heaven;
        }
      }
      cout<< -1<< endl;
      heaven:;
    }else{
      int c; cin>> c;
      a[c].clear();
    }
  }

  return 0;
}
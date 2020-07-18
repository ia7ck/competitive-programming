#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  while(true){
    int n, m, q; cin>> n>> m>> q;
    if(n==0 && m==0 && q==0) break;

    bool a[m][n];
    rep(i, m)rep(j, n) a[i][j]=true;
    string ps=string(n, '0'),
           pb=string(m, '0');
    rep(_, q){
      string s, b; cin>> s>> b;
      vector<int> changed_switches, unchanged_switches;
      rep(i, n){
        if(s[i]=='1') changed_switches.push_back(i);
        else unchanged_switches.push_back(i);
      }
      vector<int> changed_bulbs, unchanged_bulbs;
      rep(i, m){
        if(pb[i]!=b[i]) changed_bulbs.push_back(i);
        else unchanged_bulbs.push_back(i);
      }
      for(int i: changed_bulbs){
        for(int j: unchanged_switches){
          a[i][j]=false;
        }
      }
      for(int i: unchanged_bulbs){
        for(int j: changed_switches){
          a[i][j]=false;
        }
      }
      ps=s; pb=b;
    }

    rep(i, m){
      int k=-1, cnt=0;
      rep(j, n){
        if(a[i][j]){
          cnt++;
          k=j;
        }
      }
      if(cnt>1){
        cout<< "?";
      }else{
        if(0<=k && k<10) cout<< k;
        else cout<< (char)('A'+(k-10));
      }
    }cout<< endl;
  }

  return 0;
}
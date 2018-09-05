#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  string s; cin>> s;

  int m=26;
  vector<vector<int>> pos(m);
  int mx_freq=0;
  rep(i, (int)s.size()){
    pos[s[i]-'a'].push_back(i);
    mx_freq=max(mx_freq, (int)pos[s[i]-'a'].size());
  }
  int num=0;
  for(auto c: s)if((int)pos[c-'a'].size()==mx_freq) num++;
  string ans="";
  rep(i, m){ // Tの先頭のアルファベット
    if((int)pos[i].size()==mx_freq){
      for(int j=0; j<num; j++){ // Tのj文字目
        int cnt=1;
        for(int k=1; k<mx_freq; k++){
          if(pos[i][k]+j<(int)s.size()){
            if(s[pos[i][0]+j]==s[pos[i][k]+j]) cnt++;
          }
        }
        if(cnt==mx_freq){
          if((int)ans.size()<j+1 || ans>s.substr(pos[i][0], j+1)){
            ans=s.substr(pos[i][0], j+1);
          }
        }else{
          break;
        }
      }
    }
  }
  cout<< ans<< endl;

  return 0;
}

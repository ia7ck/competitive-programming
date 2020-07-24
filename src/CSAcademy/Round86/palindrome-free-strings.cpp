#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  string s; cin>> s;
  int n=(int)s.size();
  int mn=n;
  for(char c1='a'; c1<='z'; c1++){
    for(char c2='a'; c2<='z'; c2++){
      if(c1==c2) continue;
      int num=(s[0]!=c1)+(s[1]!=c2);
      string t=""; t+=c1; t+=c2;
      for(int i=2; i<n; i++){
        if(t[i-2]==s[i] or t[i-1]==s[i]){
          num++;
          t+="-";
        }else{
          t+=s[i];
        }
      }
      mn=min(mn, num);
    }
  }
  cout<< mn<< endl;
  return 0;
}

#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

bool f(const string &s){
  if(s=="ABC") return true;
  if(s.size()<=3) return false;
  int n=s.size();
  char c;
  if(s.substr(0, 3)=="ABC") c='A';
  else if(s.substr(n-3, 3)=="ABC") c='C';
  else c='B';
  string t="";
  rep(i, n){
    if(s.substr(i, 3)=="ABC"){
      t+=c;
      i+=2;
    }else{
      t+=s[i];
    }
  }
  if(s.size()==t.size()) return false;
  string u="";
  int m=t.size();
  rep(i, m){
    if(t[i]==c){
      u+="ABC";
    }else{
      u+=t[i];
    }
  }
  if(s==u) return f(t);
  else return false;
}

int main(){

  string s; cin>> s;

  bool ok=f(s);
  if(ok) cout<< "Yes"<< endl;
  else cout<< "No"<< endl;

  // cout<< ret<< endl;

  return 0;
}

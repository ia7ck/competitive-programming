#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

string f(const string &s){
  int n=s.size();
  rep(i, n){
    if(s[i]=='e'){
      if(i-1>=0 && s[i-1]=='m' && i+1<n && s[i+1]=='w'){
        if((i-1==0 && i+2<n) || (i-1>0 && i+2==n)) return s;
        else return f(s.substr(0, i-1)+s.substr(i+2));
      }
    }
  }
  return s;
}

int main(){

  string s; cin>> s;

  auto ret=f(s);
  if(ret.empty()) cout<< "Cat"<< endl;
  else cout<< "Rabbit"<< endl;

  return 0;
}
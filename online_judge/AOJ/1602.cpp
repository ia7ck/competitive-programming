#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

// o=0: add, o=1: mul
int f(const vector<string> &str, int i=0, int d=0, int o=0){
  vector<int> cand;
  for(int j=i; j<str.size(); j++){
    const string &s=str[j];
    int k=0;
    while(s[k]=='.') k++;
    if(k==d) cand.push_back(j);
    else if(k<d) break;
  }
  int ret=o;
  for(int j: cand){
    const char ch=str[j][d];
    if(o==0){
      if(ch=='+') ret+=f(str, j+1, d+1, 0);
      else if(ch=='*') ret+=f(str, j+1, d+1, 1);
      else ret+=(ch-'0');
    }else{
      if(ch=='+') ret*=f(str, j+1, d+1, 0);
      else if(ch=='*') ret*=f(str, j+1, d+1, 1);
      else ret*=(ch-'0');
    }
  }
  return ret;
}

int main(){

  while(true){
    int n; cin>> n;
    if(n==0) break;
    vector<string> str(n);
    rep(i, n) cin>> str[i];
    cout<< f(str)<< endl;
  }

  return 0;
}

/*

  *
  .+
  ..2
  ..3
  .4
  .+
  ..5
  ..6

  (2+3)*4+(5+6)

*/
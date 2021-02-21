#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int f(const string &s, int i){
  if(s[i-1]!=']'){
    int o=1, ret=0;
    for(int j=i-1; j>=0; j--){
      if(s[j]=='[') break;
      else ret+=(s[j]-'0')*o, o*=10;
    }
    return (ret-1)/2+1;
  }
  int c=0;
  vector<int> idx;
  for(int j=i-1; j>=0; j--){
    if(c<0) break;
    if(s[j]==']'){
      if(c==0) idx.push_back(j);
      c++;
    }else if(s[j]=='['){
      c--;
    }
  }
  vector<int> a;
  for(int j: idx) a.push_back(f(s, j));
  sort(a.begin(), a.end());
  int k=((int)a.size()-1)/2+1;
  int ret=0;
  rep(j, k) ret+=a[j];
  return ret;
}

int main(){

  int n; cin>> n;
  while(n--){
    string s; cin>> s;
    cout<< f(s, (int)(s.size())-1)<< endl;
  }

  return 0;
}
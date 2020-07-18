#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  string s; cin>> s;

  int t=0;
  for(int i=0; i<n; i++){
    while(i<n and ((s[i]-'0')^t)==0) i++;
    if(i<n) t^=1;
  }
  cout<< t<< endl;

  return 0;
}

// https://csacademy.com/submission/1707351/

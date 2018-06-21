#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  string s; cin>> s;

  for(int i=1; i<=n; i++)if(n%i==0){
    rep(j, i/2){
      swap(s[j], s[i-j-1]);
    }
  }

  cout<< s<< endl;

  return 0;
}
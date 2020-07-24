#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  if(n==1){
    cout<< "n"<< endl;
    return 0;
  }

  vector<string> str;
  str.push_back("a");
  int j=0;
  while(str.size()<n){
    int k=str.size();
    for(; j<k; j++){
      for(char c='a'; c<='z'; c++){
        str.push_back(str[j]+c);
        if(str.size()>=n) goto hell;
      }
    }
  }
  hell:;

  rep(i, n){
    if(i+1<n) cout<< (str[i]+"a")<< endl;
    else cout<< (str[i]+"n")<< endl;
  }

  return 0;
}
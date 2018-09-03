#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  string s="";
  vector<int> cand;
  cand.push_back(0);
  rep(i, n){
    string t; cin>> t;
    char mn='z';
    for(auto j: cand)if(t[j]<mn) mn=t[j];
    s+=mn;
    vector<int> nex;
    for(auto j: cand)if(t[j]==mn){
        if(nex.size()==0) nex.push_back(j);
        if(nex.back()<j) nex.push_back(j);
        nex.push_back(j+1);
    }
    cand.swap(nex);
  }
  cout<< s<< endl;

  return 0;
}

// https://csacademy.com/submission/1708571

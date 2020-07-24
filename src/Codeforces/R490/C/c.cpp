#include<iostream>
#include<vector>
#include<algorithm>
#include<map>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, k; cin>> n>> k;
  string s; cin>> s;

  string t=s;
  sort(t.begin(), t.end());
  map<char, int> er;
  rep(i, k) er[t[i]]++;

  string u="";
  for(char c: s){
    if(er[c]>0) er[c]--;
    else u+=c;
  }

  cout<< u<< endl;

  return 0;
}
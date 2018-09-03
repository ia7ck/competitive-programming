#include<iostream>
#include<vector>
#include<algorithm>
#include<numeric>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, t; cin>> n>> t;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  using i64=long long;
  vector<i64> freq(*max_element(a.begin(), a.end())+1, 0);
  for(auto _a: a) freq[_a]++;
  vector<i64> cnt(t+1, 0);
  for(int i=1; i<(int)freq.size(); i++){
    if(freq[i]==0) continue;
    for(int j=i; j<=t; j+=i) cnt[j]+=freq[i];
  }
  auto mx=*max_element(cnt.begin(), cnt.end());
  int num=0;
  for(int i=1; i<(int)cnt.size(); i++)if(cnt[i]==mx) num++;
  cout<< mx<< " "<< num<< endl;

  return 0;
}

// https://csacademy.com/submission/1707392

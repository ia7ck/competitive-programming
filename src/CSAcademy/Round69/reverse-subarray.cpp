#include<iostream>
#include<vector>
#include<algorithm>
#include<map>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  bool inc=true;
  for(int i=1; i<n; i++) inc&=(a[i-1]<=a[i]);
  if(inc){
    using i64=long long;
    map<int, i64> freq;
    for(int _a: a) freq[_a]++;
    i64 num=0;
    for(auto e: freq) num+=e.second*(e.second+1)/2;
    cout<< num<< endl;
    cout<< 1<< " "<< 1<< endl;
    return 0;
  }

  int cnt=0, l, r;
  for(int i=0; i<n; i++){
    int _l=i;
    while(i+1<n and a[i]>=a[i+1]) i++;
    int _r=i;
    if(_l<_r && a[_l]>a[_r]){
      l=_l; r=_r;
      cnt++;
    }
  }
  if(cnt>=2){
    cout<< 0<< endl;
    return 0;
  }
  reverse(a.begin()+l, a.begin()+r+1);
  inc=true;
  for(int i=1; i<n; i++) inc&=(a[i-1]<=a[i]);
  if(inc){
    cout<< 1<< endl<< l+1<< " "<< r+1<< endl;
  }else{
    cout<< 0<< endl;
  }

  return 0;
}
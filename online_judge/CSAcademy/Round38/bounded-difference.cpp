#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  bool ok=true;
  for(int i=1; i<n; i++) ok&=abs(a[i-1]-a[i])<=k;
  if(ok){
    cout<< 0<< endl;
    return 0;
  }

  vector<vector<int>> pos;
  for(int i=1; i<n; i++){
    if(abs(a[i-1]-a[i])>k){
      pos.push_back({i-1, i});
    }
  }
  if(pos.size()>4){
    cout<< -1<< endl;
    return 0;
  }

  auto check=[&](int i){
    bool ret=true;
    if(i-1>=0) ret&=abs(a[i-1]-a[i])<=k;
    if(i+1<n) ret&=abs(a[i+1]-a[i])<=k;
    return ret;
  };

  auto check2=[&](){
    bool ret=true;
    for(auto p: pos){
      for(auto i: p){
        ret&=check(i);
      }
    }
    return ret;
  };

  for(auto p: pos){
    for(auto i: p){
      rep(j, n){
        swap(a[i], a[j]);
        bool ok=true;
        ok&=check(i);
        ok&=check(j);
        ok&=check2();
        if(ok){
          cout<< i+1<< " "<< j+1<< endl;
          return 0;
        }
        swap(a[i], a[j]);
      }
    }
  }

  cout<< -1<< endl;

  return 0;
}

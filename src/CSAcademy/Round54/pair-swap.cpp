#include<iostream>
#include<vector>
#include<algorithm>
#include<set>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(n);
  rep(i, n) cin>> a[i];
  rep(i, n) a.push_back(1e9);

  set<pair<int, int>> s;
  rep(i, n*2){
    if(i<k){
      s.insert({a[i], -i});
    }else if(i>=n){
      if(i-k<n){
        auto mn=*s.begin();
        if(mn.first<a[i-k]){
          swap(a[i-k], a[-mn.second]);
          break;
        }
        s.erase({a[i-k], -(i-k)});
      }
    }else{
      s.insert({a[i], -i});
      auto mn=*s.begin();
      if(mn.first<a[i-k]){
        swap(a[i-k], a[-mn.second]);
        break;
      }
      s.erase({a[i-k], -(i-k)});
    }
  }

  rep(i, n) cout<< a[i]<< "\n "[i+1<n];
  return 0;
}

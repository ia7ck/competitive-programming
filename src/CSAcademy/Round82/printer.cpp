#include<iostream>
#include<vector>
#include<algorithm>
#include<numeric>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  const int n=4;
  using i64=long long;
  struct T{i64 need, price;};
  vector<T> items(n);
  rep(i, n){
    i64 a; cin>> a;
    items[i].need=a;
  }
  rep(i, n){
    i64 b; cin>> b;
    items[i].price=b;
  }
  i64 bundle; cin>> bundle;

  sort(items.begin(), items.end(), [&](const T &l, const T &r){
    return l.need==r.need ? l.price>r.price : l.need<r.need;
  });
  i64 tot=0;
  rep(i, n){
    i64 normal=0;
    for(int j=i; j<n; j++) normal+=items[i].need*items[j].price;
    if(normal>=items[i].need*bundle){
      tot+=items[i].need*bundle;
      for(int j=n-1; j>=i; j--) items[j].need-=items[i].need;
    }else{
      tot+=items[i].need*items[i].price;
      items[i].need=0;
    }
  }

  cout<< tot<< endl;
  return 0;
}
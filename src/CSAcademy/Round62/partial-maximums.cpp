#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  bool inc=true;
  for(int i=1; i<n; i++) inc&=(a[i-1]<a[i]);
  if(inc){
    cout<< n-1<< endl;
    return 0;
  }

  vector<int> b;
  for(int i=0, mx=0; i<n; i++){
    if(mx<a[i]) b.push_back(i);
    mx=max(mx, a[i]);
  }
  int mx=0, base=b.size();
  b.push_back(n);
  for(int i=0; i+1<(int)(b.size()); i++){
    int tmp=(i==0 ? 0 :a[b[i-1]]), ad=0;
    for(int j=b[i]+1; j<b[i+1]; j++){
      if(tmp<a[j]) ad++;
      tmp=max(tmp, a[j]);
    }
    mx=max(mx, base-1+ad);
  }

  cout<< mx<< endl;
  return 0;
}

// https://csacademy.com/submission/1708094/

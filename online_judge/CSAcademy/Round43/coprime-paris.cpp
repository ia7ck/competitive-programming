#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;

  auto isPrime=[&](int x){
    if(x%2==0) return false;
    for(int i=3; i*i<=x; i+=2){
      if(x%i==0) return false;
    }
    return true;
  };

  vector<int> ans;
  for(int i=5; (int)ans.size()<n; i++){
    if(isPrime(i)) ans.push_back(i);
  }
  auto r=n*(n-1)/2-k;
  int a=1;
  while(a*(a-1)/2<=r) a++;
  a--;
  rep(i, a) ans[i]*=2;
  r-=a*(a-1)/2;
  rep(i, r) ans[i]*=3;
  if(r>0) ans[n-1]*=3;

  rep(i, n) cout<< ans[i]<< (i+1<n ? " " : "\n");
  return 0;
}

// (a+1)*a/2-a*(a-1)/2=(a/2)(a+1-(a-1))=a

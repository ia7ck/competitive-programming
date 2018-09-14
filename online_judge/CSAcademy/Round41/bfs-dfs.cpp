#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  vector<int> a(n), b(n);
  rep(i, n) cin>> a[i];
  rep(i, n) cin>> b[i];

  if(n>=2 and a[1]!=b[1]){
    cout<< -1<< endl;
    return 0;
  }

  cout<< n-1+max(0, n-2)<< endl;
  for(int i=1; i<n; i++) cout<< 1<< " "<< a[i]<< endl;
  for(int i=2; i<n; i++) cout<< b[i-1]<< " "<< b[i]<< endl;

  return 0;
}

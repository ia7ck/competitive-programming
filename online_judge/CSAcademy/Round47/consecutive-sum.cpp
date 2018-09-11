#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;

  int l=1, r=1, s=0; // [l, r)の和がn未満
  while(r<n){
    s+=(r++);
    while(s>=n){
      if(s==n){
        cout<< l<< " "<< r-1<< endl;
        return 0;
      }
      s-=(l++);
    }
  }
  cout<< -1<< endl;

  return 0;
}

#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int a, b, c, n; cin>> a>> b>> c>> n;

  if(a>n or b>n or c>n){
    cout<< -1<< endl;
    return 0;
  }
  if(min(a, b)<c){
    cout<< -1<< endl;
    return 0;
  }
  int k=n-(a+b-c);
  if(k<=0) cout<< -1<< endl;
  else cout<< k<< endl;

  return 0;
}
#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, w; cin>> n>> w;

  int last=0, sum=0;
  rep(i, n){
    int t, l; cin>> t>> l;
    if(t==1){
      if((sum+=l)>=w){
        cout<< last+1<< endl;
        return 0;
      }
    }else{
      last=i+1;
      sum=0;
    }
  } 

  cout<< -1<< endl;

  return 0;
}
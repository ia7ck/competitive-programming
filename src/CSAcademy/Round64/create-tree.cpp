#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  for(int i=2; i<=n; i++){
    int ret;
    do{
      cout<< "Q "<< 1<< " "<< i<< endl;
      cin>> ret;
    }while(ret==0);
  }
  cout<< "A"<< endl;

  return 0;
}

// https://csacademy.com/submission/1707628

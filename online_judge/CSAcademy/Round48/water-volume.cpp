#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  int v=0, mn=1e9, mx=0;
  while(n--){
    int t, u; cin>> t>> u;
    if(t){
      v+=u;
    }else{
      v-=u;
    }
    mx=max(mx, v);
    mn=min(mn, v);
  }

  if(mn<0) mx+=abs(mn);
  cout<< mx<< endl;

  return 0;
}

#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  using i64=long long;
  i64 n, k; cin>> n>> k;

  i64 x=0;
  rep(_, n-1){
    x+=(x/(k-1))+1;
  }

  cout<< x <<endl;

  return 0;
}

/*

  i:   0 1 2 3 ... k-1 k k+1 k+2 ... k*2-1 k*2 k*2+1 ... k*3-1 k*3 k*3+1 ... k*4-1 k*4 k*4+1
  i+1:   0 1 2 ... k-2   k-1  k  ... k*2-3     k*2-2 ... k*3-4     k*3-3 ... k*4-5     k*4-4

  x in [(k-1)*m, (k-1)*(m+1)-1] => x+=(m+1)
  m=0, 1, 2, ...

*/
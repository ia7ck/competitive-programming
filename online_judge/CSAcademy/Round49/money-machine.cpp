#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, c, x1, u, x2;
  cin>> n>> c>> x1>> u>> x2;
  rep(i, n){
    if(u<=c and u<=(n-i)*x2){
      c-=u;
      x1+=x2;
    }
    c+=x1;
  }
  cout<< c<< endl;

  return 0;
}

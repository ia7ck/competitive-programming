#include<iostream>
#include<vector>
#include<algorithm>
#include<cmath>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;

  cout<< 0<< " "<< 0<< endl;
  const double pi=acos(-1);
  printf("%.15f %.15f\n", cos(-pi/6), sin(-pi/6));
  printf("%.15f %.15f\n", cos(pi/6), sin(pi/6));

  rep(i, (n-3)){
    auto theta=-pi/6+(pi/3/n*(i+1));
    printf("%.15f %.15f\n", cos(theta), sin(theta));
  }
  return 0;
}

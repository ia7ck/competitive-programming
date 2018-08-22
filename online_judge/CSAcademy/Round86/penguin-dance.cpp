#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, p, t; cin>> n>> p>> t;
  vector<int> moves={0, 0, 0, 0, 1, -1, 1, 1 ,1};
  int sz=(int)moves.size();
  int f=1;
  rep(i, t) f+=moves[i%sz];
  if(p<f){
    cout<< -1<< endl;
  }else if(f+n<=p){
    cout<< -1<< endl;
  }else{
    cout<< p-f+1<< endl;
  }
  return 0;
}

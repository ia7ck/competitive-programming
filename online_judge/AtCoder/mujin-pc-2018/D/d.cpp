#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int rev(int x){
  int ret=0;
  while(x>0){
    ret+=x%10;
    if((x/=10)>0) ret*=10;
  }
  return ret;
}

bool f(int x, int y){
  set<pair<int, int>> s;
  s.insert({x, y});
  while(x*y>0){
    // if(x<10 or y<10) return false;
    if(x<y){
      x=rev(x);
    }else{
      y=rev(y);
    }
    if(x<y){
      y=y-x;
    }else{
      x=x-y;
    }
    if(s.insert({x, y}).second==false) return true;
  }
  return false;
}

int main(){

  int n, m; cin>> n>> m;
  long long tot=0;
  for(int x=1; x<=n; x++)for(int y=1; y<=m; y++)if(f(x, y)) tot++;
  cout<< tot<< endl;

  return 0;
}
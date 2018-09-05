#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int t; cin>> t;
  while(t--){
    vector<int> d(3);
    rep(i, 3) cin>> d[i];
    sort(d.begin(), d.end());
    auto r=d[0]+d[1]-d[2];
    if(r>=0){
      if(r%2==0){
        cout<< "0 0"<< " "<< d[2]<< " "<< 0<< " "<< d[0]-r/2<< " "<< r/2<< endl;
      }else{
        cout<< -1<< endl;
      }
    }else{
      cout<< -1<< endl;
    }
  }

  return 0;
}

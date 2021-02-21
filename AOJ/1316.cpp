#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  while(true){
    int h, w; cin>> h>> w;
    if(h==0 && w==0) break;
    vector<string> a(h);
    rep(i, h) cin>> a[i];
    string best="";
    set<string> dict;
    const int dx[]={-1, -1, -1,  0, 0,  1, 1, 1};
    const int dy[]={-1,  0,  1, -1, 1, -1, 0, 1};
    rep(i, h)rep(j, w){
      rep(k, 8){
        int x=i, y=j;
        string t="";
        do{
          t+=a[x][y];
          if(dict.count(t)){
            if(best.size()<t.size()){
              best=t;
            }else if(best.size()==t.size()){
              if(t<best) best=t;
            }
          }else if(t.size()>=2){
            dict.insert(t);
          }
          x=(x+h+dx[k])%h;
          y=(y+w+dy[k])%w;
        }while(x!=i || y!=j);
      }
    }

    if(best.size()>=2) cout<< best<< endl;
    else cout<< 0<< endl;
  }

  return 0;
}
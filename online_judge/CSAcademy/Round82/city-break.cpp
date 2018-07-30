#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n, s; cin>> n>> s;
  vector<int> d(n);   
  rep(i, n) cin>> d[i];

  s--;
  int l=s, r=s;
  int sum=0, tot=0;
  rep(_, n-1){
    int ll=(l-1+n)%n, rr=(r+1)%n;
    int dl=d[ll], dr=d[r];
    if(l==s) dr+=sum;
    if(r==s) dl+=sum;
    tot+=min(dl, dr);
    if(dl==dr){
      if(ll<rr){
        sum+=d[ll];
        l=s=ll;
      }else{
        sum+=d[r];
        r=s=rr;
      }
    }else if(dl<dr){
      sum+=d[ll];
      l=s=ll;
    }else{
      sum+=d[r];
      r=s=rr;
    }
  }

  cout<<tot<< endl;
  return 0;
}
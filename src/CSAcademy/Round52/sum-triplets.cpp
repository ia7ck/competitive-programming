#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  vector<int> freq(5001, 0);
  for(auto x: a) freq[x]++;
  long long tot=0;
  for(int x=0; x<=5000; x++)if(freq[x]){
   for(int y=x; y<=5000; y++)if(freq[y]){
      if(x==0 and y==0){
        long long k=freq[0];
        tot+=k*(k-1)*(k-2)/6;
      }else if(x*y==0){
        long long k=freq[max(x, y)];
        tot+=freq[0]*k*(k-1)/2;
      }else if(x==y){
        if(x*2<=5000){
          long long k=freq[x];
          tot+=freq[x*2]*k*(k-1)/2;
        }
      }else{
        if(x+y<=5000){
          tot+=(long long)freq[x]*freq[y]*freq[x+y];
        }
      }
    }
  }
  cout<< tot<< endl;

  return 0;
}

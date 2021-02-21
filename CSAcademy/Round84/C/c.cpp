#include<iostream>
#include<vector>
#include<algorithm>
#include<map>
#include<cassert>
using namespace std;

#define rep(i,n) for(int i=0;i<(n);i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];  

  sort(a.begin(), a.end());
  vector<int> b;
  for(int e: a){
    if(b.size()>0 and b.back()==e) b.pop_back();
    else b.push_back(e);
  }
  vector<int> odd;
  for(int e: b)if(e%2==1) odd.push_back(e);
  if(odd.size()%2==1){
    cout<< -1<< endl;
    return 0;
  }

  cout<< b.size()<< endl;

  for(int i=0; i<odd.size(); i+=2){
    cout<< odd[i+1]<< " "<< odd[i]<< endl;
    assert((odd[i+1]-odd[i])>0);
    assert((odd[i+1]-odd[i])/2>0);
    cout<< (odd[i+1]-odd[i])<< " "<< (odd[i+1]-odd[i])/2<< endl;
  }

  for(int e: b)if(e%2==0){
    cout<< e<< " "<< e/2<< endl; 
  }

  return 0;
}
#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  if(2<=n && n<=3 && k==0){
    cout<< -1<< endl;
    return 0;
  }

  struct E{int a, b, c;};
  vector<E> edges;
  for(int i=1; i<n; i++) edges.push_back(E{i, i+1, i});
  int r=n-k-1;
  rep(i, r){
    if(i+3<=n){
      edges.push_back(E{1, i+3, n+i});
    }else{
      edges.push_back(E{2, n, 1000000000});
    }
  }

  cout<< edges.size()<< endl;
  for(auto e: edges) cout<< e.a<< " "<< e.b<< " "<< e.c<< endl;

  return 0;
}

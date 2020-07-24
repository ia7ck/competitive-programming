#include<iostream>
#include<vector>
#include<algorithm>
#include<deque>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  vector<int> d(n-1);
  for(int i=1; i<n; i++){
    d[i-1]=a[i]-a[i-1];
  }
  deque<int> q;
  rep(i, n-k-1){
    while(q.size()>0 and d[q.back()]<d[i]) q.pop_back();
    q.push_back(i);
  }
  auto mndiff=d[q.front()];
  for(int i=n-k-1; i+1<n; i++){
    if(q.front()==i-(n-k-1)) q.pop_front();
    while(q.size()>0 and d[q.back()]<d[i]) q.pop_back();
    q.push_back(i);
    mndiff=min(mndiff, d[q.front()]);
  }
  cout<< mndiff<< endl;

  return 0;
}

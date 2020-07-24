#include<iostream>
#include<vector>
#include<algorithm>
#include<set>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n, k; cin>> n>> k;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  multiset<int> s;
  for(int i=1; i<n-k; i++){
    s.insert((a[i]-a[i-1])*(-1));
  }
  int mndiff=(*s.begin())*(-1);
  for(int i=n-k; i<n; i++){
    auto itr=s.find((a[i-(n-k)+1]-a[i-(n-k)])*(-1));
    s.erase(itr);
    s.insert((a[i]-a[i-1])*(-1));
    mndiff=min(mndiff, (*s.begin())*(-1));
  }
  cout<< mndiff<< endl;

  return 0;
}

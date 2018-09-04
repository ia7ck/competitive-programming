#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  vector<int> a(n);
  rep(i, n) cin>> a[i];

  vector<int> st;
  int tot=0;
  rep(i, n){
    while(st.size()>0 and st.back()>a[i]){
      st.pop_back();
      tot++;
    }
    if(st.size()==0 || st.back()<a[i]) st.push_back(a[i]);
  }
  cout<< tot+(int)st.size()<< endl;

  return 0;
}

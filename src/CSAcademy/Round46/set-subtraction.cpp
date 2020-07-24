#include<iostream>
#include<vector>
#include<algorithm>
#include<map>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  int n; cin>> n;
  vector<int> a(n*2);
  rep(i, n*2) cin>> a[i];

  sort(a.begin(), a.end());
  for(int i=1; i<n*2; i++){
    int d=a[i]-a[0];
    if(d==0) continue;
    vector<int> used(n*2, false);
    used[0]=used[i]=true;
    map<int, int> freq;
    for(int j=1; j<n*2; j++){
      if(i!=j){
        freq[a[j]]++;
      }
    }
    vector<int> ans={a[i]};
    for(int j=1; j<n*2; j++){
      if(i==j) continue;
      if(freq[a[j]]>0 and freq[a[j]-d]>0){
        freq[a[j]]--;
        freq[a[j]-d]--;
        ans.push_back(a[j]);
        if((int)ans.size()==n){
          cout<< d<< endl;
          rep(i, n){
            cout<< ans[i]<< (i+1<n ? " " : "\n");
          }
          return 0;
        }
      }
    }
  }

  cout<< -1<< endl;

  return 0;
}

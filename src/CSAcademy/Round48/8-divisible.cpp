#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int main(){

  string s; cin>> s;

  if(s.size()<=3){
    int ans=1000;
    vector<int> idx(s.size());
    rep(i, idx.size()) idx[i]=i;
    do{
      string t="";
      for(auto j: idx) t+=s[j];
      auto num=stoi(t);
      if(to_string(num).size()!=t.size()) continue;
      if(num%8==0) ans=min(ans, num);
    }while(next_permutation(idx.begin(), idx.end()));
    if(ans<1000){
      cout<< ans<< endl;
    }else{
      cout<< -1<< endl;
    }
    return 0;
  }

  string ans="";
  rep(_, 2000) ans+="9";
  vector<int> freq(10, 0);
  for(char c: s) freq[c-'0']++;
  rep(x, 10){
    if(freq[x]==0) continue;
    freq[x]--;
    rep(y, 10){
      if(freq[y]==0) continue;
      freq[y]--;
      rep(z, 10){
        if(freq[z]==0) continue;
        freq[z]--;
        string t="";
        t+=(x+'0'); t+=(y+'0'); t+=(z+'0');
        if(stoi(t)%8==0){
          string u="";
          rep(i, 10){
            rep(_, freq[i]){
              u+=(i+'0');
            }
          }
          if(u[0]!='0'){
            ans=min(ans, u+t);
          }else if(u.back()!='0'){
            rep(i, u.size()){
              if(u[i]!='0'){
                swap(u[0], u[i]);
                ans=min(ans, u+t);
                break;
              }
            }
          }
        }
        freq[z]++;
      }
      freq[y]++;
    }
    freq[x]++;
  }
  if(ans.size()<=1000){
    cout<< ans<< endl;
  }else{
    cout<< -1<< endl;
  }

  return 0;
}

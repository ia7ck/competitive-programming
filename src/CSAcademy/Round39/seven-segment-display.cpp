#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>

using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

int count(string s){
  vector<int> seg={6, 2, 5, 5, 4, 5, 6, 3, 7, 6};
  int ret=0;
  for(char c: s) ret+=seg[c-'0'];
  return ret;
}

int main(){

  int k; cin>> k;

  if(k==1){
    cout<< -1<< endl;
    return 0;
  }
  if(k==3){
    cout<< 7<< endl;
    return 0;
  }
  if(k==4){
    cout<< 4<< endl;
    return 0;
  }
  if(k==6){
    cout<< 0<< endl;
    return 0;
  }

  int r=k%7;
  string s="";
  if(r==0){
    rep(_, k/7) s+="8";
  }else if(r==1){
    s+="10";
    rep(_, (k-8)/7) s+="8";
  }else if(r==2){
    s+="1";
    rep(_, (k-2)/7) s+="8";
  }else if(r==3){
    if(k==10){
      s="22";
    }else{
      s+="200";
      rep(_, (k-17)/7) s+="8";
    }
  }else if(r==4){
    s+="20";
    rep(_, (k-11)/7) s+="8";
  }else if(r==5){
    s+="2";
    rep(_, (k-5)/7) s+="8";
  }else if(r==6){
    s+="6";
    rep(_, (k-6)/7) s+="8";
  }

  assert(count(s)==k);
  cout<< s<< endl;

  return 0;
}

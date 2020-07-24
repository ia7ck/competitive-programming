#include<iostream>
#include<vector>
#include<algorithm>
#include<set>
using namespace std;
#define rep(i,n) for(int i=0;i<(n);i++)

uint32_t x = 0, y = 1, z = 2, w = 3;
uint32_t generate() { 
    uint32_t t = (x^(x<<11));
    x = y;
    y = z;
    z = w;
    w = (w ^ (w >> 19)) ^ (t ^ (t >> 8)); 
    return w;
}

int main(void) {
    uint32_t seed; cin >> seed; 
    x = seed;
    int n=10000001;

    long long yes=0, no=4294967296;
    while(no-yes>1){
      auto m=(yes+no)/2;
      x=seed; y=1; z=2; w=3;
      int cnt=0;
      rep(_, n){
        auto a=generate();
        if(a<m) cnt++;
      }
      (cnt<=n/2 ? yes : no) = m;
    }

    cout<< yes<< endl;
    return 0;
}
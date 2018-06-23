void main(){
  import std.algorithm;

  const long mod=998244353;
  const int M=1_000_00;
  static fact=new long[](M); // コンパイル時に
  static inv_fact=new long[](M);
  { // init
    fact[0]=fact[1]=1;
    foreach(i; 2..M) fact[i]=i*fact[i-1]%mod;
    long powmod(long a, long x){
      if(x==0) return 1;
      else if(x==1) return a;
      else if(x&1) return a*powmod(a, x-1)%mod;
      else return powmod(a*a%mod, x/2);
    }
    foreach(i; 0..M) inv_fact[i]=powmod(fact[i], mod-2);
  }
  long comb(long nn, long rr){
    if(nn<rr) return 0;
    long ret=fact[nn]%mod;
    (ret*=inv_fact[rr])%=mod;
    (ret*=inv_fact[nn-rr])%=mod;
    return ret;
  }

  assert(comb(4, 0)==1);
  assert(comb(4, 2)==6);
  assert(comb(4, 4)==1);
  assert(comb(4, 5)==0);
}

/*
  https://beta.atcoder.jp/contests/agc025/submissions/2611132

  todo: https://beta.atcoder.jp/contests/abc034/tasks/abc034_c
*/
void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  const int m=40;
  const long mod=1e9.to!long+7;
  auto mat=new long[][][](m, 2, 2);
  mat[0][0][0]=mat[0][0][1]=1;
  mat[0][1][0]=1; mat[0][1][1]=0;
  foreach(t; 1..m){
    auto prev=mat[t-1], cur=mat[t];
    foreach(i; 0..2)foreach(j; 0..2){
      foreach(k; 0..2) (cur[i][j]+=prev[i][k]*prev[k][j])%=mod;
    }
  }

  long n; rd(n);
  if(n==1){
    writeln(1);
    return;
  }
  n--;
  auto a=new long[](2);
  a[0]=1;
  foreach(t; 0..m)if(n&(1L<<t)){
    auto b=new long[](2);
    foreach(i; 0..2)foreach(j; 0..2) (b[i]+=mat[t][i][j]*a[j]%mod)%=mod;
    a.swap(b);
  }
  writeln(a[0]*(a[0]+a[1])%mod);
}

/*

  f_n      1 1  f_{n-1}
  f_{n-1}  1 0  f_{n-2}

  f_1   E 000
  f_2   T 001
  f_3 T^2 010

*/

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
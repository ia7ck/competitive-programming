void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m; rd(n, m);
  long sum=0;
  while(m--){
    long x, d; rd(x, d);
    sum+=x*n;
    if(d>0){// 0+d+d*2+...+d*(n-1)
      sum+=d*n*(n-1)/2;
    }else{
      auto k=n/2;
      sum+=d*k*(k+1)/2*2;
      if(n%2==0) sum-=d*k;
    }
  }

  writefln("%.18f", 1.0*sum/n);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

/*

  0 0
  -1 2
  -1 2
  -2 -3

  0 0 0
  0 2 4
  5 7 9

*/
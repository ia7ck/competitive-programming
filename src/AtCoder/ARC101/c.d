void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k; rd(n, k);
  auto a=readln.split.to!(long[]);

  long[] b, c;
  foreach(x; a){
    if(x<0) b~=-x;
    else if(x==0) k--;
    else c~=x;
  }
  b.sort!"a<b";
  if(k==0){
    writeln(0);
    return;
  }

  long f(){
    if(b.length==0 && c.length==0){
      return 0;
    }
    if(b.length==0){
      return c[k-1];
    }
    long ret=1_000_000_000_000_000_000;
    if(k-1<c.length){
      ret=min(ret, c[k-1]);
    }
    foreach(i; 0..k){
      if(i<b.length && 0<=k-i-2 && k-i-2<c.length){
        ret=min(ret, b[i]*2+c[k-i-2]);
      }
    }
    return ret;
  }

  auto ans=f();
  swap(b, c);
  ans=min(ans, f());
  writeln(ans);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

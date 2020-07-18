import std.stdio, std.string, std.conv, std.algorithm;

void main(){

  int n; rd(n);
  long a, b; rd(a, b);
  long[] cand;
  for(int i=2; i*i<=a; i++){
    if(a%i==0){
      cand~=i;
      while(a%i==0) a/=i;
    }
  }
  if(a>1) cand~=a;
  for(int i=2; i*i<=b; i++){
    if(b%i==0){
      cand~=i;
      while(b%i==0) b/=i;
    }
  }
  if(b>1) cand~=b;
  foreach(i; 0..(n-1)){
    long s, t; rd(s, t);
    long[] nex;
    foreach(x; cand){
      if(s%x==0 || t%x==0) nex~=x;
    }
    cand.swap(nex);
  }

  if(cand.length>0){
    writeln(cand[0]);
  }else{
    writeln(-1);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=new int[](n);
  foreach(i; 0..n) rd(a[i]);

  auto s=a.sum;
  if(s%10>0){
    writeln(s);
    return;
  }
  auto all=reduce!((r, e)=>r&&(e%10==0))(true, a);
  if(all){
    writeln(0);
    return;
  }
  int mn=1000000000;
  foreach(e; a){
    if(e%10>0){
      mn=min(mn, e);
    }
  }
  import std.exception;
  if(mn<1000000000){
    writeln(s-mn);
  }else{
    enforce(false);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, q; rd(n, q);
  int s=0;
  while(q--){
    int t, k; rd(t, k);
    if(t){
      (s+=k)%=n;
    }else{
      writeln(((k-1)+s)%n+1);
    }
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto w=new int[](n), h=new int[](n);
  foreach(i; 0..n) rd(w[i], h[i]);

  int cur=1_000_000_000+1;
  foreach(i; 0..n){
    if(cur<min(w[i], h[i])){
      writeln("NO");
      return;
    }
    if(cur>=max(w[i], h[i])) cur=max(w[i], h[i]);
    else cur=min(w[i], h[i]);
  }

  writeln("YES");

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
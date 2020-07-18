void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m, q; rd(n, m, q);
  auto s=readln.chomp.to!(char[]);
  auto t=readln.chomp.to!(char[]);

  auto ok=new bool[](s.length+1);
  for(int i=0; i+t.length<=s.length; i++){
    if(s[i..(i+t.length)]==t) ok[i]=true;
  }
  while(q--){
    int l, r; rd(l, r);
    l--; r--;
    int w=0;
    for(int i=l; i<=r; i++){
      if(ok[i] && i+t.length-1<=r) w++;
    }
    writeln(w);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
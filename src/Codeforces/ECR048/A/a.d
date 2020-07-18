void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; long m; rd(n, m);
  auto as=readln.split.to!(long[]);

  long s=0;
  long[] ans;
  foreach(a; as){
    s+=a;
    ans~=s/m;
    s%=m;
  }

  writefln("%(%s %)", ans);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
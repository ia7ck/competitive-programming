void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  struct T{int l, r;}
  auto s=new T[](n);
  foreach(i; 0..n) rd(s[i].l, s[i].r);

  int mxlen=0;
  s.sort!"a.l==b.l ? a.r<b.r : a.l>b.l";
  // writeln(s[0]);
  mxlen=max(mxlen, s[1..$].minElement!("a.r").r-s[1..$].maxElement!("a.l").l);
  s.sort!"a.r==b.r ? a.l>b.l : a.r<b.r";
  // writeln(s[0]);
  mxlen=max(mxlen, s[1..$].minElement!("a.r").r-s[1..$].maxElement!("a.l").l);
  writeln(mxlen);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

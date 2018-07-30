void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, x; rd(n, x);
  auto a=readln.split.to!(int[]);
  
  int[int] bef, aft;
  foreach(e; a){
    if(e in bef) bef[e]++;
    else bef[e]=1;
    if ((e&x) in aft) aft[e&x]++;
    else aft[e&x]=1;
  }
  int mn=3;
  foreach(e; a){
    if(bef[e]>1) mn=0;
    else{
      bef[e]--;
      if(((e&x) in bef) && bef[e&x]>0) mn=min(mn, 1);
      if(aft[e&x]>1) mn=min(mn, 2);
      bef[e]++;
    }
  }
  if(mn>2) writeln(-1);
  else writeln(mn);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
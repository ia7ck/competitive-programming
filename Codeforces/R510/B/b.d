void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto mns=new int[](8);
  fill(mns, 1000000000);
  foreach(_; 0..n){
    auto args=readln.split.to!(char[][]);
    int kind=0;
    foreach(c; args[1]){
      kind^=(1<<(c-'A'));
    }
    mns[kind]=min(mns[kind], args[0].to!(int));
  }
  struct T{int kind, price;}
  T[] as, bs, cs;
  foreach(bit; 1..(1<<3)){
    if(mns[bit]==1000000000) continue;
    if(bit&(1<<0)) as~=T(bit, mns[bit]);
    if(bit&(1<<1)) bs~=T(bit, mns[bit]);
    if(bit&(1<<2)) cs~=T(bit, mns[bit]);
  }
  as~=T(0, 0); bs~=T(0, 0); cs~=T(0, 0);
  int mincost=1000000000;
  foreach(a; as){
    foreach(b; bs){
      foreach(c; cs){
        int bit=a.kind|b.kind|c.kind;
        if(bit==((1<<3)-1)){
          mincost=min(mincost, a.price+b.price+c.price);
        }
      }
    }
  }
  if(mincost<1000000000){
    writeln(mincost);
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

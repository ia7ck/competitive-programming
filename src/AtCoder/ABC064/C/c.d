void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(int[]);

  auto num=new int[](9);
  foreach(e; a){
    foreach(i; 0..8){
      if(i*400<=e && e<(i+1)*400){
        num[i]++; break;
      }
    }
    if(e>=3200){
      num[$-1]++;
    }
  }
  auto lb=reduce!((r, e)=>(r+(e>0 ? 1 : 0)))(0, num[0..$-1]);
  if(num[$-1]==0){
    writeln(lb, " ", lb);
  }else{
    if(lb==0){
      writeln(1, " ", num[$-1]);
    }else{
      writeln(lb, " ", lb+num[$-1]);
    }
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

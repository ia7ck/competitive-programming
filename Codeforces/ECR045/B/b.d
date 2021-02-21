void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k; rd(n, k);
  auto a=readln.split.to!(int[]);

  int num=0, c=1;
  sort(a);
  // writeln(a);
  foreach(i; 1..n){
    if(a[i-1]==a[i]){
      c++;
    }else{
      if(a[i-1]+k<a[i]) num+=c;
      c=1;
    }
  }
  // writeln(num);
  // writeln(c);
  writeln(num+c);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
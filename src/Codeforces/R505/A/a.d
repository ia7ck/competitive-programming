import std.stdio, std.string, std.conv, std.algorithm;

void main(){

  int n; rd(n);
  auto s=readln.chomp.to!(char[]);

  if(n==1){
    writeln("Yes");
    return;
  }

  bool[char] set;
  foreach(c; s){
    if(c in set){
      writeln("Yes");
      return;
    }
    set[c]=true;
  }

  writeln("No");

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

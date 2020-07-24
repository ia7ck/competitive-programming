void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto s=readln.chomp.to!(char[]);

  int f=0;
  int k=0;
  foreach(i; 0..n){
    if(s[i]=='('){
      k++;
    }else{
      if(k==0){
        f++;
      }else{
        k--;
      }
    }
  }
  foreach(_; 0..f) write("(");
  write(s);
  foreach(_; 0..k) write(")");
  writeln();

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

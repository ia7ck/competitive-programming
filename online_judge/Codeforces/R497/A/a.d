void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s=readln.chomp.to!(char[]);

  bool[char] v;
  v['a']=true;
  v['i']=true;
  v['u']=true;
  v['e']=true;
  v['o']=true;

  int n=s.length.to!(int);
  foreach(i; 0..n){
    if((s[i] in v)==null){
      if(s[i]=='n'){
        // 
      }else{
        if(i+1<n){
          if((s[i+1] in v)==null){
            writeln("NO");
            return;
          }
        }else{
          writeln("NO");
          return;
        }
      }
    }else{
      // 
    }
  }

  writeln("YES");
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
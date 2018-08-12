void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n, m; rd(n, m);
  auto a=readln.split.to!(long[]).map!((e)=>(e%m)).array;

  auto cul=new long[](n+1);
  foreach(i; 0..n) cul[i+1]=(cul[i]+a[i])%m;
  long[long] freq;
  freq[0]=0;
  long tot=0;
  foreach(x; cul[1..$]){
    if(x==0){
      tot+=(++freq[x]);
      continue;
    }
    if(x in freq){
      tot+=freq[x]++;
    }else{
      freq[x]=1;
    }
  }
  writeln(tot);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

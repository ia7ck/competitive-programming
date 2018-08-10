void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.random, std.array;

  // const int n=10;
  // auto rnd=Random(unpredictableSeed);
  // auto a=new char[](n), b=new char[](n);
  // foreach(i; 0..n){
  //   a[i]=(uniform!"[]"(0, 1, rnd)+'0').to!(char);
  //   b[i]=(uniform!"[]"(0, 1, rnd)+'0').to!(char);
  // }
  
  int n; rd(n);
  auto a=readln.chomp.to!(char[]);
  auto b=readln.chomp.to!(char[]);

  long solve(){
    long p, q, r, s;
    foreach(i; 0..n){
      if(a[i]=='0' && b[i]=='0') p++;
      else if(a[i]=='0' && b[i]=='1') q++;
      else if(a[i]=='1' && b[i]=='0') r++;
      else s++;
    }
    return p*r+p*s+q*r;
  }

  long brute(){
    auto _a=a.map!((e)=>(e-'0')).array, _b=b.map!((e)=>(e-'0')).array;
    long ret=0;
    foreach(i; 0..n)foreach(j; 0..i){
      int o1=_a[i]|_b[i], o2=_a[j]|_b[j];
      if(o1!=(_a[j]|_b[i]) || o2!=(_a[i]|_b[j])) ret++;
    }
    return ret;
  }

  // if (solve()!=brute()){
  //   writeln(a);
  //   writeln(b);
  //   writeln(solve());
  //   writeln(brute());
  // }

  writeln(solve());
}

/*
  0 0 p
  0 1 q
  1 0 r
  1 1 s
*/

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
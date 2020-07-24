void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int h, w; rd(h, w);
  auto s=new char[][](h, w);
  foreach(i; 0..h) s[i]=readln.chomp.to!(char[]);

  long f(int _h, int _w, char[][] _s){
    long ret=0;
    auto a=new int[](_w);
    foreach(i; 0.._h){
      int last=-1;
      foreach(j; 0.._w){
        if(_s[i][j]=='#') last=j;
        a[j]=max(a[j], i);
        if(a[j]==i){
          while(_s[a[j]][j]=='.' && a[j]+1<_h && _s[a[j]+1][j]=='.') a[j]++;
        }
        if(j-last-1>0) ret+=(a[j]-i)*(j-last-1);
      }
    }
    return ret;
  }

  long tot=0;
  tot+=f(h, w, s);
  auto t=new char[][](h, w);
  foreach(i; 0..h)foreach(j; 0..w) t[i][j]=s[h-i-1][w-j-1];
  tot+=f(h, w, t);
  auto u=new char[][](w, h);
  foreach(i; 0..w)foreach(j; 0..h) u[i][j]=s[h-j-1][i];
  tot+=f(w, h, u);
  auto v=new char[][](w, h);
  foreach(i; 0..w)foreach(j; 0..h) v[i][j]=u[w-i-1][h-j-1];
  tot+=f(w, h, v);

  writeln(tot);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
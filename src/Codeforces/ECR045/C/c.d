void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n; rd(n);
  long[int] freq;
  auto y=new int[](n);
  auto ng=new bool[](n);
  foreach(i; 0..n){
    auto s=readln.chomp.to!(char[]);
    char[] st;
    foreach(c; s){
      if(c=='('){
        st~='(';
      }else{
        if(st.length>0){
          if(st.back=='(') st=st[0..($-1)];
          else st~=')';
        }else{
          st~=')';
        }
      }
    }
    int x=0;
    foreach(c; st){
      if(c=='(') x++;
      else x--;
    }
    if(st.length>=2 && (st[0]==')' && st[$-1]=='(')){ng[i]=true; continue;}
    y[i]=x;
    if(x in freq) freq[x]++;
    else freq[x]=1;
    hell:;
  }
  
  long tot=0;
  foreach(i; 0..n)if(ng[i]==false){
    if(y[i]>=0 && (-y[i] in freq)){
      tot+=freq[-y[i]];
    }
    // writeln(tot);
  }
  
  writeln(tot);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
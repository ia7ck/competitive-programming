void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  struct Student{
    int id, sum;
  }
  int n; rd(n);
  auto students=new Student[](n);
  foreach(i; 0..n){
    auto s=readln.split.to!(int[]).reduce!"a+b";
    students[i]=Student(i, s);
  }
  students.sort!("a.sum==b.sum ? a.id<b.id : a.sum>b.sum");
  foreach(i, student; students){
    if(student.id==0){
      writeln(i+1);
      return;
    }
  }

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
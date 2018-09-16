#include<vector>
#include<limits>
#include<queue>

using namespace std;

template<typename T>
struct Edge{
  int to;
  T cost;
};

template<typename T>
vector<T> dijkstra(vector<vector<Edge<T>>> &g, int s){
  const auto inf=numeric_limits<T>::max();
  vector<T> dist((int)g.size(), inf);
  dist[s]=0;
  priority_queue<pair<T, int>> q;
  q.push({-dist[s], s});
  while(q.size()>0){
    T cost; int u;
    tie(cost, u)=q.top(); q.pop();
    cost*=-1;
    for(auto e: g[u]){
      if(cost+e.cost<dist[e.to]){
        dist[e.to]=cost+e.cost;
        q.push({-dist[e.to], e.to});
      }
    }
  }
  return dist;
}

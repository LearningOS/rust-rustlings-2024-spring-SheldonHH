#include <iostream>
#include <vector>
#include <unordered_set>
using namespace std;

class Graph {
private:
    vector<vector<int>> adj; // 邻接表

    void dfsUtil(int v, unordered_set<int>& visited, vector<int>& visitOrder) {
        // 标记当前节点为已访问
        visited.insert(v);
        // 将当前节点添加到访问顺序中
        visitOrder.push_back(v);

        // 遍历当前节点的每个邻接节点
        for (int adjNode : adj[v]) {
            // 如果邻接节点未被访问，则递归地访问它
            if (visited.find(adjNode) == visited.end()) {
                dfsUtil(adjNode, visited, visitOrder);
            }
        }
    }

public:
    Graph(int n) : adj(n) {} // 构造函数，初始化邻接表的大小

    void addEdge(int src, int dest) {
        adj[src].push_back(dest); // 添加边
        adj[dest].push_back(src); // 假设是无向图，因此也要添加反向边
    }

    vector<int> dfs(int start) {
        unordered_set<int> visited; // 存储已访问节点
        vector<int> visitOrder; // 访问顺序
        dfsUtil(start, visited, visitOrder); // 执行DFS
        return visitOrder;
    }
};

// 为了演示，我们在main函数中使用Graph类
int main() {
    // 创建一个简单的图
    Graph g(5);
    g.addEdge(0, 1);
    g.addEdge(0, 2);
    g.addEdge(3, 4);

    // 从节点0开始的DFS
    vector<int> visitOrder = g.dfs(0);
    cout << "从节点0开始的DFS: ";
    for (int v : visitOrder) cout << v << " ";
    cout << endl;

    // 从节点3开始的DFS，展示一个断开的图
    visitOrder = g.dfs(3);
    cout << "从节点3开始的DFS: ";
    for (int v : visitOrder) cout << v << " ";
    cout << endl;

    return 0;
}

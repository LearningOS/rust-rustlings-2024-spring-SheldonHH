class Graph:
    def __init__(self, n):
        self.adj = [[] for _ in range(n)]  # 初始化邻接表

    def add_edge(self, src, dest):
        # 添加边
        self.adj[src].append(dest)
        self.adj[dest].append(src)  # 假设是无向图，因此也要添加反向边

    def dfs_util(self, v, visited, visit_order):
        # 标记当前节点为已访问
        visited.add(v)
        # 将当前节点添加到访问顺序中
        visit_order.append(v)

        # 遍历当前节点的每个邻接节点
        for adj_node in self.adj[v]:
            # 如果邻接节点未被访问，则递归地访问它
            if adj_node not in visited:
                self.dfs_util(adj_node, visited, visit_order)

    def dfs(self, start):
        visited = set()  # 存储已访问节点
        visit_order = []  # 访问顺序
        self.dfs_util(start, visited, visit_order)  # 执行DFS
        return visit_order

# 为了演示，我们在这里使用Graph类
if __name__ == "__main__":
    # 创建一个简单的图
    g = Graph(5)
    g.add_edge(0, 1)
    g.add_edge(0, 2)
    g.add_edge(3, 4)

    # 从节点0开始的DFS
    visit_order = g.dfs(0)
    print("从节点0开始的DFS:", visit_order)

    # 从节点3开始的DFS，展示一个断开的图
    visit_order = g.dfs(3)
    print("从节点3开始的DFS:", visit_order)

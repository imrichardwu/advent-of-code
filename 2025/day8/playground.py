import math

class UnionFind:
    def __init__(self, n):
        self.parent = {}
        self.rank = {}
        self.num_components = n

        for i in range(1, n + 1):
            self.parent[i] = i
            self.rank[i] = 0
    
    # Find parent of n, with path compression.
    def find(self, n):
        parent = self.parent[n]
        while parent != self.parent[parent]:
            self.parent[parent] = self.parent[self.parent[parent]]
            parent = self.parent[parent]
        return parent

    # Union by height / rank.
    # Return false if already connected, true otherwise.
    def union(self, n1, n2):
        p1, p2 = self.find(n1), self.find(n2)
        if p1 == p2:
            return False
        
        if self.rank[p1] > self.rank[p2]:
            self.parent[p2] = p1
        elif self.rank[p1] < self.rank[p2]:
            self.parent[p1] = p2
        else:
            self.parent[p1] = p2
            self.rank[p2] += 1
        self.num_components -= 1
        return True

class Solution:
    def p1(self, points, numConnections=1000):
        edges = []

        for i in range(len(points)):
            x1, y1, z1 = points[i]
            for j in range(i + 1, len(points)):
                x2, y2, z2 = points[j]
                # euclidean distance
                dist = math.sqrt((x1 - x2)**2 + (y1 - y2)**2 + (z1 - z2)**2)
                edges.append((dist, i + 1, j + 1))

        edges.sort()
        uf = UnionFind(len(points))
        processed = 0

        for dist, u, v in edges:
            processed += 1
            uf.union(u, v)  # Try to connect, even if already in same circuit
            # Process exactly numConnections edges (some may be skipped)
            if processed == numConnections:
                break

        groups = {} 

        for i in range(1, len(points) + 1):
            root = uf.find(i)
            groups[root] = groups.get(root, 0) + 1

        sizes = sorted(groups.values(), reverse=True)
        while len(sizes) < 3:
            sizes.append(1)
        return sizes[0] * sizes[1] * sizes[2]
    
    def p2(self, points):
        edges = []

        for i in range(len(points)):
            x1, y1, z1 = points[i]
            for j in range(i + 1, len(points)):
                x2, y2, z2 = points[j]
                dist = math.sqrt((x1 - x2)**2 + (y1 - y2)**2 + (z1 - z2)**2)
                edges.append((dist, i, j))

        edges.sort()
        uf = UnionFind(len(points))

        for dist, i, j in edges:
            if uf.union(i + 1, j + 1):  # Only count successful unions
                if uf.num_components == 1:  # All connected!
                    # Return product of X coordinates
                    return points[i][0] * points[j][0]
        
        return -1  # Should never reach here

if __name__ == "__main__":
    sol = Solution()
    points = []
    with open("input.txt", 'r') as file:
        for line in file:
            line = line.strip()
            if not line:
                continue
            x, y, z = map(int, line.split(','))
            points.append((x, y, z))
    result = sol.p2(points)
    print(result)
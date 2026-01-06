from collections import defaultdict

class Solution:
    def p1(self, edges):
        graph = defaultdict(list)
        for src, dests in edges:
            for dest in dests:
                graph[src].append(dest)
        
        visit = set()
        res = [0]

        def dfs(node):
            if node == 'out':
                res[0] += 1
                return
            
            visit.add(node)

            for nei in graph[node]:
                if nei not in visit:
                    dfs(nei)
            visit.remove(node)

        dfs('you')
        return res[0]
    
    def p2(self, edges):
        graph = defaultdict(list)
        for src, dests in edges:
            for dest in dests:
                graph[src].append(dest)
        
        # Memoization cache: (node, has_dac, has_fft) -> count
        cache = {}
        
        def dfs(node, has_dac, has_fft):
            # Base case: reached 'out'
            if node == 'out':
                return 1 if (has_dac and has_fft) else 0
            
            # Check memoization
            state = (node, has_dac, has_fft)
            if state in cache:
                return cache[state]
            
            # Update flags based on current node
            new_has_dac = has_dac or (node == 'dac')
            new_has_fft = has_fft or (node == 'fft')
            
            # Count paths from this node
            count = 0
            for nei in graph[node]:
                count += dfs(nei, new_has_dac, new_has_fft)
            
            # Memoize and return
            cache[state] = count
            return count
        
        return dfs('svr', False, False)

if __name__ == "__main__":
    sol = Solution()
    edges = []
    with open('input.txt') as file:
        for line in file:
            src, dist = line.strip().split(':')
            dist = dist.strip().split()
            edges.append([src, dist])

    # edges = [
    #     ['aaa', ['you', 'hhh']],
    #     ['you', ['bbb', 'ccc']],
    #     ['bbb', ['ddd', 'eee']],
    #     ['ccc', ['ddd', 'eee', 'fff']],
    #     ['ddd', ['ggg']],
    #     ['eee', ['out']],
    #     ['fff', ['out']],
    #     ['ggg', ['out']],
    #     ['hhh', ['ccc', 'fff', 'iii']],
    #     ['iii', ['out']]
    # ]
    # edges = [
    #     ['svr', ['aaa', 'bbb']],
    #     ['aaa', ['fft']],
    #     ['fft', ['ccc']],
    #     ['bbb', ['tty']],
    #     ['tty', ['ccc']],
    #     ['ccc', ['ddd', 'eee']],
    #     ['ddd', ['hub']],
    #     ['hub', ['fff']],
    #     ['eee', ['dac']],
    #     ['dac', ['fff']],
    #     ['fff', ['ggg', 'hhh']],
    #     ['ggg', ['out']],
    #     ['hhh', ['out']]
    # ]
    print(sol.p2(edges))
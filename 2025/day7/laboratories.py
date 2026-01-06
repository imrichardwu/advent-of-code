class Solution:
    def p1(self, matrix):
        ROWS, COLS = len(matrix), len(matrix[0])
        split = [0]
        visit = set()
        def dfs(row, col):
            if row < 0 or row >= ROWS or col < 0 or col >= COLS or (row, col) in visit:
                return
            
            visit.add((row, col))
            if matrix[row][col] == '^':
                split[0] += 2
                dfs(row + 1, col - 1)
                dfs(row + 1, col + 1)
            else: 
                dfs(row + 1, col)
                

        for row in range(ROWS):
            for col in range(COLS):
                if matrix[row][col] == 'S':
                    dfs(row, col)
                    break
        
        return split[0]
    
    def main(self, matrix):
        ROWS, COLS = len(matrix), len(matrix[0])
        cache = {}
        
        def dfs(row, col):
            if row < 0 or row >= ROWS or col < 0 or col >= COLS:
                return 1
            if (row, col) in cache:
                return cache[(row, col)]
                
            
            if matrix[row][col] == '^':
                left = dfs(row + 1, col - 1)
                right = dfs(row + 1, col + 1)
                cache[(row, col)] = left + right
            else: 
                cache[(row, col)] = dfs(row + 1, col)
            
            return cache[(row, col)]

        for row in range(ROWS):
            for col in range(COLS):
                if matrix[row][col] == 'S':
                    return dfs(row, col)
        

if __name__ == "__main__":
    sol = Solution()
    matrix = []
    with open("input.txt", "r") as file:
        for line in file:
            matrix.append(list(line.strip()))
    
    # strs = """  .......S.......
    #             ...............
    #             .......^.......
    #             ...............
    #             ......^.^......
    #             ...............
    #             .....^.^.^.....
    #             ...............
    #             ....^.^...^....
    #             ...............
    #             ...^.^...^.^...
    #             ...............
    #             ..^...^.....^..
    #             ...............
    #             .^.^.^.^.^...^.
    #             ..............."""
    # for line in strs.splitlines():
    #     matrix.append(list(line.strip()))

    print(sol.main(matrix))
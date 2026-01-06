class Solution:
    def main(self, grid):
        ROWS, COLS = len(grid), len(grid[0]) 
        res = 0

        directions = [
            (1, 0), (0, 1), (-1, 0), (0, -1),  # cardinal
            (1, 1), (1, -1), (-1, 1), (-1, -1)  # diagonal
        ]
        
        while True:
            visit = set()
            
            for row in range(ROWS):
                for col in range(COLS):
                    if grid[row][col] == '@':
                        count = 0
                        for dr, dc in directions:
                            nr, nc = row + dr, col + dc
                            if (0 <= nr < ROWS and 0 <= nc < COLS and 
                                grid[nr][nc] == '@'):  # Count adjacent '@'
                                count += 1

                        if count < 4:
                            visit.add((row, col))
            if not visit:
                break

            for r, c in visit:
                grid[r][c] = '.'
                res += 1

        return res
    
if __name__ == "__main__":
    sol = Solution()
    grid = []
    with open("input.txt", "r") as file:
        for line in file:
            grid.append(list(line.strip()))
    print(sol.main(grid))
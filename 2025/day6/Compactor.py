class Solution:
    def p1(self, lines):
        # Split into top (number lines) and bottom (operator line)
        topLine = [line.rstrip() for line in lines[:-1]]
        bottomLine = lines[-1].rstrip()
        
        # Count number of problems from first line
        nums = len(topLine[0].split()) if topLine else 0
        
        # Collect numbers for each problem
        nums = [[] for _ in range(nums)]
        for line in topLine:
            for idx, s in enumerate(line.split()):
                nums[idx].append(int(s))
        
        # Extract operators from bottom line (filter out whitespace)
        operations = [c for c in bottomLine if c in ['+', '*']]
        
        # Calculate result
        res = 0
        for problem_nums, operation in zip(nums, operations):
            if operation == '+':
                total = sum(problem_nums)
            elif operation == '*':
                total = 1
                for num in problem_nums:
                    total *= num
            res += total
        
        return res
    
    def main(self, lines):
        # Split into top (number lines) and bottom (operator line)
        topLine = lines[:-1]
        bottomLine = lines[-1]
        
        # Find maximum width
        max_width = max(len(line.rstrip()) for line in topLine)
        
        # Pad all lines to same width
        grid = []
        for line in topLine:
            row = list(line.rstrip())
            while len(row) < max_width:
                row.append(' ')
            grid.append(row)
        
        ROWS = len(grid)
        COLS = max_width
        
        # Process columns from left to right
        problems = []
        current = []
        
        for col in range(COLS):
            # Collect digits from top to bottom in this column
            digits = []
            for row in range(ROWS):
                char = grid[row][col]
                
                if char.isdigit():
                    digits.append(int(char))
            
            if digits:
                # Build number: acc * 10 + d for each digit (most significant at top)
                num = 0
                for digit in digits:
                    num = num * 10 + digit
                current.append(num)
            else:
                # Empty column - end of problem
                if current:
                    problems.append(current)
                    current = []
        
        # Don't forget the last problem
        if current:
            problems.append(current)
        
        # Extract operators from bottom line
        operations = [c for c in bottomLine if c in ['+', '*']]
        
        # Calculate result
        res = 0
        for problem, operation in zip(problems, operations):
            if operation == '+':
                total = sum(problem)
            elif operation == '*':
                total = 1
                for num in problem:
                    total *= num
            res += total
        
        return res
                   
if __name__ == "__main__":
    sol = Solution()
    
    # Test with example
    example_lines = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  "
    ]
    result = sol.main(example_lines)

    # with open("input.txt", "r") as file:
    #     lines = file.readlines()
    #     print(lines)
    #     result = sol.p1(lines)
    #     print(result)

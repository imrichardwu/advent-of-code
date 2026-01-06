class Solution:
    def main(self, battery, k):
        stack = []
        remove = len(battery) - k

        for char in battery:
            while remove > 0 and stack and stack[-1] < char:
                stack.pop()
                remove -= 1
            stack.append(char)

        # Keep only k digits
        stack = stack[:k]
        
        return int("".join(stack)) if stack else 0

if __name__ == '__main__':
    sol = Solution()
    res = 0
    with open('input.txt', 'r') as file:
        for line in file:
            line = line.strip()
            if line:
                res += sol.main(line, 12)
    print(res)
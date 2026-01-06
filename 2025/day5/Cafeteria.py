class Solution:
    def p1(self, intervals, product_id):
        intervals.sort(key=lambda interval: interval[0])

        overlapping_interval = [intervals[0]]
        for i in range(1, len(intervals)):
            start, end = intervals[i]
            if start <= overlapping_interval[-1][1]:
                overlapping_interval[-1][1] = max(overlapping_interval[-1][1], end)
            else:
                overlapping_interval.append(intervals[i])
        
        intervals = overlapping_interval
        res = 0
        for product in product_id:
            for i in range(len(intervals)):
                start, end = intervals[i]
                if start <= product <= end:
                    res += 1
                    break
        return res
    
    def p2(self, intervals, product_id):
        intervals.sort(key=lambda interval: interval[0])

        overlapping_interval = [intervals[0]]
        for i in range(1, len(intervals)):
            start, end = intervals[i]
            if start <= overlapping_interval[-1][1]:
                overlapping_interval[-1][1] = max(overlapping_interval[-1][1], end)
            else:
                overlapping_interval.append(intervals[i])
        
        intervals = overlapping_interval
        res = 0
        for i in range(len(intervals)):
            start, end = intervals[i]
            res += (end - start + 1)
        return res
        
if __name__ == "__main__":
    solution = Solution()
    intervals = []
    product_id = []
    line_stop = None
    with open("input.txt", "r") as file:
        for i, line in enumerate(file):
            if line != '\n':
                start, end = map(int, line.strip().split('-'))
                intervals.append([start, end])
            else:
                break

        for line in file:
            line = line.strip()
            if line != ' ':
                product_id.append(int(line))
    
    print(solution.p2(intervals, product_id))

    # intervals = [[3, 5], [10, 14], [16, 20], [12, 18]]
    # product_id = [5, 8, 11, 17, 32]
    # print(solution.p2(intervals, product_id))
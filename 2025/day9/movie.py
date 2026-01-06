from shapely.geometry import Polygon, box

class Solution:
    def main(self, points):
        n = len(points)
        best = 0
        
        # Create polygon for Part 2
        polygon = Polygon(points)

        for i in range(n):
            x1, y1 = points[i]
            for j in range(i + 1, n):
                x2, y2 = points[j]
                
                # Calculate inclusive width and height
                width = abs(x1 - x2) + 1
                height = abs(y1 - y2) + 1
                area = width * height
                
                # Part 2: Check if rectangle is inside polygon
                rect = box(min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2))
                if polygon.contains(rect) and area > best:
                    best = area

        return best


if __name__ == "__main__":
    solution = Solution()
    # points = [(7,1), (11,1), (11,7), (9,7), (9,5), (2,5), (2,3), (7,3)]
    points = []
    with open("input.txt", "r") as file:
        for line in file:
            x, y = map(int, line.strip().split(','))
            points.append((x, y))

    print(solution.main(points))
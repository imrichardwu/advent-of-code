

class Region:
    def __init__(self, width, length, counts):
        self.width = width
        self.length = length
        self.counts = counts
    
    def definitely_fits(self):
        blocks_free = (self.width // 3) * (self.length // 3)
        blocks_to_place = sum(self.counts)
        return blocks_free >= blocks_to_place

if __name__ == "__main__":
    presents = []
    grid = []
    with open("input.txt") as file:
        # read the whole file
        data = file.read().strip().split('\n\n')
        presentShapes = data[:-1]
        gridInfo = data[-1]
        for present in presentShapes:
            p = present.split('\n')
            presents.append(p[1:])
        
        for line in gridInfo.split('\n'):
            size, fill = line.split(':')
            n, m = map(int, size.strip().split('x'))
            counts = list(map(int, fill.strip().split())) # counts of each present
            region = Region(n, m, counts)
            grid.append(region)
    
    res = 0
    for region in grid:
        res += 1 if region.definitely_fits() else 0
    print(res)
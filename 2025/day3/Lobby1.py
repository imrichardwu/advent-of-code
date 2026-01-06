def main(battery):
    res = ''
    left = 0
    maxNum = float('-inf')
    for right in range(1, len(battery)):
        if int(battery[left] + battery[right]) > maxNum:
            res = battery[left] + battery[right]
            maxNum = int(res)
        if int(battery[right]) > int(battery[left]):
            left = right
    
    return int(res) if res else 0

if __name__ == "__main__":
    res = 0
    with open("input.txt", "r") as file:
        for line in file:
            line = line.strip()
            if line:
                res += main(line)
    print(res)
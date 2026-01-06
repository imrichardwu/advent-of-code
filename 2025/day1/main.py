def main(rotations):
    point = 50
    res = 0
    for rotation in rotations:
        distance = int(rotation[1:])
        
        if rotation[0] == 'L':
            # Count how many clicks land on 0
            # We hit 0 when (point - k) % 100 == 0, i.e., k % 100 == point % 100
            # So k = point, point+100, point+200, ...
            # Count how many such k are in range [1, distance]
            if point > 0 and distance >= point:
                # We definitely hit 0 at least once
                res += 1 + (distance - point) // 100
            elif point == 0 and distance >= 100:
                # Starting at 0, we hit it again after 100 clicks
                res += distance // 100
            
            point = (point - distance) % 100
            
        elif rotation[0] == 'R':
            # Count how many clicks land on 0
            # We hit 0 when (point + k) % 100 == 0, i.e., k % 100 == (100 - point) % 100
            # So k = 100-point, 100-point+100, 100-point+200, ...
            if point > 0:
                target = 100 - point
                if distance >= target:
                    res += 1 + (distance - target) // 100
            elif point == 0 and distance >= 100:
                # Starting at 0, we hit it again after 100 clicks
                res += distance // 100
            
            point = (point + distance) % 100
    
    print(res)

if __name__ == "__main__":
    rotations = []
    with open("input.txt", "r") as file:
        for line in file:
            rotations.append(line.strip())
    main(rotations)
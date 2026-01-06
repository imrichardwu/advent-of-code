# def main(start, end):
#     res = 0

#     for product_id in range(start, end + 1):
#         # Check if ID is a sequence repeated twice
#         str_id = str(product_id)
        
#         # Must have even length to be repeatable
#         if len(str_id) % 2 == 0:
#             mid = len(str_id) // 2
#             first_half = str_id[:mid]
#             second_half = str_id[mid:]
            
#             # If both halves are identical, it's an invalid ID
#             if first_half == second_half:
#                 res += product_id  # Add the actual ID value, not just count
    
#     return res
 
def main(start, end):
    res = 0

    for product_id in range(start, end + 1):
        str_id = str(product_id)
        
        # Try all possible pattern lengths (from 1 to half the string length)
        for pattern_len in range(1, len(str_id) // 2 + 1):
            # Check if the string length is divisible by pattern length
            if len(str_id) % pattern_len == 0:
                pattern = str_id[:pattern_len]
                # Check if repeating this pattern gives us the full string
                if pattern * (len(str_id) // pattern_len) == str_id:
                    res += product_id
                    break  # Found a valid repeat pattern, no need to check more
    return res

if __name__ == "__main__":
    product_id_range = []
    with open("input.txt", "r") as file:
        content = file.read().strip()
        # Split by comma to get each range
        ranges = content.split(",")
        for range_str in ranges:
            start, end = map(int, range_str.split("-"))
            product_id_range.append((start, end))
    res = 0
    for start, end in product_id_range:
        res += main(start, end)
    print(res)
from collections import deque
import pulp

class Solution:
    def p1(self, lights, button):
        goal = 0
        for i, light in enumerate(lights):
            if light == 1:
                goal |= (1 << i)

        # Precompute button masks for efficiency
        masks = []
        for group in button:
            mask = 0
            # Create a bitmask for each button group
            for i in group:
                mask |= (1 << i) # Set the i-th bit
            masks.append(mask)

        queue = deque([(0, 0)])  # (current_state, steps)
        seen = set([0])
        
        # BFS to find the minimum button presses
        while queue:
            current, steps = queue.popleft() # current state and number of steps

            if current == goal:
                return steps   

            for mask in masks:
                next_state = current ^ mask # Toggle the bits using XOR
                if next_state not in seen:
                    seen.add(next_state)
                    queue.append((next_state, steps + 1))

        return 0
    
    def p2(self, buttons, jolts):
        # Create ILP model
        model = pulp.LpProblem("min_jolt_presses", pulp.LpMinimize)

        # One integer variable per button
        presses = []
        for i in range(len(buttons)):
            var = pulp.LpVariable(f"press_{i}", lowBound=0, cat=pulp.LpInteger)
            presses.append(var)

        # Objective function
        model += pulp.lpSum(presses)

        # Constraints, one per jolt index
        for j, target in enumerate(jolts):
            contributing = []
            for k, group in enumerate(buttons):
                if j in group:
                    contributing.append(presses[k])
            # Sum of all button presses that touch this jolt index must equal target
            model += pulp.lpSum(contributing) == target

        # Solve model
        model.solve(pulp.PULP_CBC_CMD(msg=False))

        # Get rounded integer
        total = 0
        for var in presses:
            total += int(round(pulp.value(var)))

        return total



if __name__ == "__main__":
    solution = Solution()
    total_presses = 0
    
    with open("input.txt", "r") as file:
        for line in file:
            lightStr, rest = line.strip().split(" ", 1)
            buttonStr, joltStr = rest.rsplit(" ", 1)

            lights = []
            for char in lightStr:
                if char == "#":
                    lights.append(1)
                elif char == ".":
                    lights.append(0)

            buttons = []
            for group in buttonStr.split():
                group = group.strip("()")
                if group:
                    nums = [int(num) for num in group.split(",")]
                    buttons.append(nums)

            jolts = []
            for jolt in joltStr.strip('{}').split(","):
                jolts.append(int(jolt))

            presses = solution.p2(buttons, jolts)
            total_presses += presses

    print(total_presses)
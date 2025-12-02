# Copy this template

DAY = "03"
INPUT_FILE = f"../input/{DAY}.txt"

TEST_INPUT = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""


def parse_grid(input_text):
    return input_text.split('\n')

def find_symbol_indices(grid, n, m):
    result = []
    for i in range(n):
        for j in range(m):
            if grid[i][j] != '.' and not grid[i][j].isdigit():
                result.append([i, j])
    return result

def find_gear_candidate_indices(grid, n, m):
    result = []
    for i in range(n):
        for j in range(m):
            if grid[i][j] == '*':
                result.append([i, j])
    return result

def part1(input_text):
    grid = parse_grid(input_text)
    n = len(grid)
    m = len(grid[0])
    visited = []
    for i in range(n):
        visited.append([False] * m)

    result = 0
    
    symbol_indices = find_symbol_indices(grid, n, m)
    coord_deltas = [[x, y] for x in [-1, 0, 1] for y in [-1, 0, 1]]
    for symbol_index in symbol_indices:
        for delta in coord_deltas:
            cur_index = [symbol_index[0] + delta[0], symbol_index[1] + delta[1]]
            if visited[cur_index[0]][cur_index[1]]:
                continue
            if grid[cur_index[0]][cur_index[1]].isdigit():
                number_left_bound = cur_index[1]
                while number_left_bound > 0 and grid[cur_index[0]][number_left_bound - 1].isdigit():
                    number_left_bound -= 1
                number_right_bound = cur_index[1]
                while number_right_bound < m - 1 and grid[cur_index[0]][number_right_bound + 1].isdigit():
                    number_right_bound += 1
                
                current_number = 0
                current_power = 0
                for j in range(number_right_bound, number_left_bound - 1, -1):
                    current_number += (10 ** current_power) * int(grid[cur_index[0]][j])
                    visited[cur_index[0]][j] = True
                    current_power += 1
                result += current_number

    return result

def part2(input_text):
    grid = parse_grid(input_text)
    n = len(grid)
    m = len(grid[0])
    result = 0
    gear_candidate_indices = find_gear_candidate_indices(grid, n, m)
    coord_deltas = [[x, y] for x in [-1, 0, 1] for y in [-1, 0, 1]]
    for gear_candidate_index in gear_candidate_indices:
        adjacent_numbers = []
        visited = []
        for i in range(3):
            visited.append([False] * 3)
        for delta in coord_deltas:
            cur_index = [gear_candidate_index[0] + delta[0], gear_candidate_index[1] + delta[1]]
            if visited[delta[0] + 1][delta[1] + 1]:
                continue
            if grid[cur_index[0]][cur_index[1]].isdigit():
                number_left_bound = cur_index[1]
                while number_left_bound > 0 and grid[cur_index[0]][number_left_bound - 1].isdigit():
                    number_left_bound -= 1
                number_right_bound = cur_index[1]
                while number_right_bound < m - 1 and grid[cur_index[0]][number_right_bound + 1].isdigit():
                    number_right_bound += 1
                
                current_number = 0
                current_power = 0
                for j in range(number_right_bound, number_left_bound - 1, -1):
                    current_number += (10 ** current_power) * int(grid[cur_index[0]][j])
                    if cur_index[0] == gear_candidate_index[0] + delta[0] and j <= gear_candidate_index[1] + 1 and j >= gear_candidate_index[1] - 1:
                        visited[delta[0] + 1][j - gear_candidate_index[1] + 1] = True
                    current_power += 1
                adjacent_numbers.append(current_number)
        if len(adjacent_numbers) == 2:
            result += adjacent_numbers[0] * adjacent_numbers[1]

    return result

def assert_equals(expected, actual):
    assert expected == actual, f"Objects not equal, expected: {expected}. actual: {actual}"

if __name__ == "__main__":
    print("=== Part 1 ===")

    assert_equals(4361, part1(TEST_INPUT))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part1(input_file_contents)
        print(f"Result = {result}")

    print("=== Part 2 ===")

    assert_equals(467835, part2(TEST_INPUT))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part2(input_file_contents)
        print(f"Result = {result}")

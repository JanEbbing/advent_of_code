# Copy this template

DAY = "01"
INPUT_FILE = f"../input/{DAY}.txt"

TEST_INPUT = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""

TEST_INPUT_2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""

def part1(input_text):
    result = 0
    for line in input_text.split('\n'):
        cur_num = 0
        for c in line:
            if c.isdigit():
                cur_num = 10 * int(c)
                break
        for c in reversed(line):
            if c.isdigit():
                cur_num += int(c)
                break
        result += cur_num
    return result

def part2(input_text):
    numbers = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
    result = 0
    for line in input_text.split('\n'):
        if not line:
            continue
        # print(f"Line {line}\nFind result: {line.find("two")}")
        cur_num = 0
        first_number_indices = list(map(lambda n: line.find(n), numbers))
        first_filtered_indices = list(filter(lambda i: first_number_indices[i] != -1, range(9)))
        last_number_indices = list(map(lambda n: line.rfind(n), numbers))
        last_filtered_indices = list(filter(lambda i: last_number_indices[i] != -1, range(9)))
        if not first_filtered_indices:
            for c in line:
                if c.isdigit():
                    cur_num = 10 * int(c)
                    break
            for c in reversed(line):
                if c.isdigit():
                    cur_num += int(c)
                    break
            result += cur_num
            continue
        first_written_number_index = min(first_filtered_indices, key=lambda i: first_number_indices[i])
        last_written_number_index = max(last_filtered_indices, key=lambda i: last_number_indices[i])
        # print(f"New line processed. Number indices {number_indices}, first written number index: {first_written_number_index}, last written number index: {last_written_number_index}")

        found_num = False
        for i in range(len(line)):
            c = line[i]
            if c.isdigit():
                found_num = True
                if i < first_number_indices[first_written_number_index]:
                    cur_num = 10 * int(c)
                else:
                    cur_num = 10 * (first_written_number_index + 1)
                break
        if not found_num:
            cur_num = 10 * (first_written_number_index + 1)
            found_num = False

        # print(f"Cur num 10 digit: {cur_num}")

        for i in range(len(line)-1, -1, -1):
            c = line[i]
            if c.isdigit():
                found_num = True
                if i > last_number_indices[last_written_number_index]:
                    cur_num += int(c)
                else:
                    cur_num += last_written_number_index + 1
                break
        if not found_num:
            cur_num += last_written_number_index + 1
            found_num = False
        # print(f"Cur num update: {cur_num}")

        result += cur_num
    return result

def assert_equals(expected, actual):
    assert expected == actual, f"Objects not equal, expected: {expected}. actual: {actual}"

if __name__ == "__main__":
    print("=== Part 1 ===")

    assert_equals(142, part1(TEST_INPUT))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part1(input_file_contents)
        print(f"Result = {result}")

    print("=== Part 2 ===")

    assert_equals(281, part2(TEST_INPUT_2))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part2(input_file_contents)
        print(f"Result = {result}")

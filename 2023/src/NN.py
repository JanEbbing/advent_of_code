# Copy this template

DAY = "NN" # TODO: Fill the day
INPUT_FILE = f"../input/{DAY}.txt"

TEST_INPUT = """
""" # TODO: Add the test input

def part1(input_text):
    # TODO: Solve Part 1 of the puzzle
    return len(input_text)

def part2(input_text):
    # TODO: Solve Part 2 of the puzzle
    return len(input_text)

def assert_equals(expected, actual):
    assert expected == actual, f"Objects not equal, expected: {expected}. actual: {actual}"

if __name__ == "__main__":
    print("=== Part 1 ===")

    # TODO: Set the expected answer for the test input
    assert_equals(0, part1(TEST_INPUT))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part1(input_file_contents)
        print(f"Result = {result}")

    # print("=== Part 2 ===")

    # # TODO: Set the expected answer for the test input
    # assert_equals(0, part2(TEST_INPUT))
    # with open(INPUT_FILE, 'r') as file:
    #     input_file_contents = file.read()
    #     result = part2(input_file_contents)
    #     print(f"Result = {result}")

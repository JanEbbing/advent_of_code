# Copy this template

DAY = "04"
INPUT_FILE = f"../input/{DAY}.txt"

TEST_INPUT = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""

def parse_cards(input_text):
    result = []
    # card ID, [winning_nums], [my_nums]
    for line in input_text.split('\n'):
        tokens = line.split(': ')
        card_id = int(tokens[0].split(' ')[-1])
        number_tokens = tokens[1].split(' | ')
        winning_nums = list(map(lambda t: int(t), filter(lambda f: f != '', number_tokens[0].split(' '))))
        my_nums = list(map(lambda t: int(t), filter(lambda f: f != '', number_tokens[1].split(' '))))
        result.append([card_id, winning_nums, my_nums])
    return result

def get_winning_nums(winning_nums, my_nums):
    winning_set = set(winning_nums)
    result = list(filter(lambda n: n in winning_set, my_nums))
    return result

def part1(input_text):
    cards = parse_cards(input_text)
    result = 0
    for card in cards:
        winning_nums = get_winning_nums(card[1], card[2])
        if winning_nums:
            result += int(2**(len(winning_nums) - 1))
    return result

def part2(input_text):
    # TODO: Solve Part 2 of the puzzle
    return len(input_text)

def assert_equals(expected, actual):
    assert expected == actual, f"Objects not equal, expected: {expected}. actual: {actual}"

if __name__ == "__main__":
    print("=== Part 1 ===")

    assert_equals(13, part1(TEST_INPUT))
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

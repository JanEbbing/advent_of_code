# Copy this template

DAY = "02"
INPUT_FILE = f"../input/{DAY}.txt"

TEST_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

def parse_games(input_text):
    # Game ID, [(red, green, blue)]
    games = []
    for line in input_text.split('\n'):
        tokens = line.split(': ')
        game_id = int(tokens[0].split(' ')[1])
        game_tokens = tokens[1].split('; ')
        dice_nums_per_game = []
        for game_line in game_tokens:
            cur_dice_nums = [0, 0, 0]
            dice_entries = game_line.split(', ')
            for entry in dice_entries:
                entry_tokens = entry.split(' ')
                num_dice = int(entry_tokens[0])
                color = entry_tokens[1]
                if color == 'red':
                    cur_dice_nums[0] = num_dice
                elif color == 'green':
                    cur_dice_nums[1] = num_dice
                elif color == 'blue':
                    cur_dice_nums[2] = num_dice
                else:
                    print(f"ERROR. invalid state, color {color}")

            dice_nums_per_game.append(cur_dice_nums)
        games.append((game_id, dice_nums_per_game))
    return games

def is_valid_game(dice_nums, available_dice):
    return all(map(lambda d: d[0] <= available_dice[0] and d[1] <= available_dice[1] and d[2] <= available_dice[2], dice_nums))

def part1(input_text):
    games = parse_games(input_text)
    available_dice = (12, 13, 14)
    result = 0
    for game in games:
        if is_valid_game(game[1], available_dice):
            result += game[0]
    return result

def get_power_of_game(dice_nums):
    cur_minimum_dice = [0, 0, 0]
    for dice_in_game in dice_nums:
        for i in range(3):
            cur_minimum_dice[i] = max(cur_minimum_dice[i], dice_in_game[i])
    return cur_minimum_dice[0] * cur_minimum_dice[1] * cur_minimum_dice[2]

def part2(input_text):
    games = parse_games(input_text)
    result = 0
    for game in games:
        result += get_power_of_game(game[1])
    return result

def assert_equals(expected, actual):
    assert expected == actual, f"Objects not equal, expected: {expected}. actual: {actual}"

if __name__ == "__main__":
    print("=== Part 1 ===")

    assert_equals(8, part1(TEST_INPUT))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part1(input_file_contents)
        print(f"Result = {result}")

    print("=== Part 2 ===")

    assert_equals(2286, part2(TEST_INPUT))
    with open(INPUT_FILE, 'r') as file:
        input_file_contents = file.read()
        result = part2(input_file_contents)
        print(f"Result = {result}")

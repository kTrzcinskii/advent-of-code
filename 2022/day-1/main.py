import easy
import hard

def get_input() -> list[list[int]]:
    elves_calories = []
    single_elf = []
    while True:
        data = input()
        if data == "end":
            break
        if data == "":
            elves_calories.append(single_elf)
            single_elf = []
        else:
            single_elf.append(int(data))
    return elves_calories

if __name__ == "__main__":
    numbers = get_input()
    biggest_easy = easy.find_biggest_easy(numbers)
    biggest_hard = hard.find_biggest_hard(numbers)
    print(f"easy: {biggest_easy}\nhard: {biggest_hard}")

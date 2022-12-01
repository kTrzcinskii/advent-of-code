def get_input() -> list[str]:
    bin_numbers = []
    while(True):
        data = input()
        if data == "end":
            break
        bin_numbers.append(data)
    
    return bin_numbers

def bin2dec(num: str) -> int:
    result = 0
    for i in range(0, len(num)):
        if num[i] == '1':
            result += pow(2, len(num) - i - 1)

    return result

def get_oxygen_rating(bin_numbers: list[str]) -> int:
    bit_index = 0
    ones_counter = 0
    most_common = ''
    while len(bin_numbers) != 1:
        for number in bin_numbers:
            if number[bit_index] == '1':
                ones_counter += 1
        max_len = len(bin_numbers)
        if ones_counter >= max_len / 2:
            most_common = '1'
        else:
            most_common = '0'
        i = 0
        while i < len(bin_numbers):
            if bin_numbers[i][bit_index] != most_common:
                bin_numbers.pop(i)
                i -= 1
            i += 1
        bit_index += 1
        ones_counter = 0

    num = bin_numbers[0]
    return bin2dec(num)

def get_co2_scrubber_rating(bin_numbers: list[str]) -> int:
    bit_index = 0
    ones_counter = 0
    least_common = ''
    while len(bin_numbers) != 1:
        for number in bin_numbers:
            if number[bit_index] == '1':
                ones_counter += 1
        max_len = len(bin_numbers)
        if ones_counter < max_len / 2:
            least_common = '1'
        else: 
            least_common = '0'
        i = 0
        while i < len(bin_numbers):
            if bin_numbers[i][bit_index] != least_common:
                bin_numbers.pop(i)
                i -= 1
            i += 1
        
        bit_index += 1
        ones_counter = 0

    num = bin_numbers[0]
    return bin2dec(num)

if __name__ == "__main__":
    bin_numbers = get_input()
    oxygen_rating = get_oxygen_rating(bin_numbers.copy())
    co2_scrubber_rating = get_co2_scrubber_rating(bin_numbers)
    print("res: ", oxygen_rating * co2_scrubber_rating)
def find_biggest_hard(nums: list[list[int]]) -> int:
    max_sum = 0
    second_max_sum = 0
    third_max_sum = 0
    for numbers in nums:
        sum = 0
        for num in numbers:
            sum += num
        print(sum)
        if sum > max_sum:
            third_max_sum = second_max_sum
            second_max_sum = max_sum
            max_sum = sum
        elif sum > second_max_sum:
            third_max_sum = second_max_sum
            second_max_sum = sum
        elif sum > third_max_sum:
            third_max_sum = sum

    return max_sum + second_max_sum + third_max_sum
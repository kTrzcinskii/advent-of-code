def find_biggest_easy(nums: list[list[str]]) -> int:
    max_sum = 0
    for numbers in nums:
        sum = 0
        for num in numbers:
            sum += num
        print(sum)
        if sum > max_sum:
            max_sum = sum

    return max_sum
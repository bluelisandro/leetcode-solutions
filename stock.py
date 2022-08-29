def stock(prices: list[int]) -> int:
    left, right = 0, len(prices) - 1
    diff = abs(prices[left] - prices[right])
    # diff = 0
    print("diff" , diff)
    
    
    while left != right:
        temp_diff = abs(prices[left] - prices[right])
        print("left: " , left , " right: " , right , " temp_diff: " , temp_diff)
        if temp_diff > diff:
            return temp_diff
        left += 1
        right -= 1

    if left > right:
        return 0
    else:
        return diff

print(stock([7,6,4,3,1]))
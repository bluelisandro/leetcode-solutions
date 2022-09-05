# Begin with the mid element of the whole array as a search key.
# If the value of the search key is equal to the item then return an index of the search key.
# Or if the value of the search key is less than the item in the middle of the interval, narrow the interval to the lower half.
# Otherwise, narrow it to the upper half.
# Repeatedly check from the second point until the value is found or the interval is empty.

def binary_search(nums, target):
    left, right = 0, len(nums) - 1
    
    while left <= right:
        mid = (right - left) // 2 
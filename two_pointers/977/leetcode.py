class Solution:
    def sortedSquares(self, nums: list[int]) -> list[int]:
        n = len(nums)
        left = 0
        right = n - 1
        k = n - 1
        result = [0] * n

        while left <= right:
            if abs(nums[left]) > abs(nums[right]):
                result[k] = nums[left] ** 2
                left += 1
            else:
                result[k] = nums[right] ** 2
                right -= 1
            k -= 1

        return result

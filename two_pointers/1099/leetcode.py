class Solution:
    def twoSumLessThanK(self, nums: list[int], k: int) -> int:
        biggest = -1
        nums.sort()
        left = 0
        right = len(nums) - 1

        while left < right:
            s = nums[left] + nums[right]
            if s < k:
                biggest = max(biggest, s)
                left += 1
            else:
                right -= 1
        return biggest

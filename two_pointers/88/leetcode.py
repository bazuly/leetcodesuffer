class Solution:
    def merge(self, nums1: list[int], m: int, nums2: list[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        # последний настоящий элемент nums1
        i = m - 1
        # последний настоящий элемент nums2
        j = n - 1
        k = m + n - 1  # индекс куда nums1 куда будет вставляться nums2
        while i >= 0 and j >= 0:
            if nums1[i] > nums2[j]:
                nums1[k] = nums2[i]
                i -= 1
            else:
                nums1[k] = nums2[j]
                j -= 1
            k -= 1
        while j >= 0:
            nums1[k] = nums2[j]
            j -= 1
            k -= 1

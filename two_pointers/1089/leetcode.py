class Solution:
    def duplicateZeros(self, arr: list[int]) -> None:
        """
        Do not return anything, modify arr in-place instead.
        """
        n = len(arr)
        out = []

        for num in arr:
            out.append(num)
            if num == 0:
                out.append(0)
        for i in range(n):
            arr[i] = out[i]

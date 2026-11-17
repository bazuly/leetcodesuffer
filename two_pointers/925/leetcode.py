class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        i = 0
        j = 0

        while i < len(name) and j < len(typed):
            if name[i] != typed[j]:
                return False

            char = name[i]
            count_name = 0
            while i < len(name) and name[i] == char:
                i += 1
                count_name += 1

            count_typed = 0
            while j < len(typed) and typed[j] == char:
                j += 1
                count_typed += 1

            if count_typed < count_name:
                return False

        return i == len(name) and j == len(typed)

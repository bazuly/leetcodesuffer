class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        i = j = 0
        result = []
        flag = True

        while i < len(word1) and j < len(word2):
            if flag:
                result.append(word1[i])
                flag = False
                i += 1
            else:
                result.append(word2[j])
                flag = True
                j += 1
        result.extend(word1[i:])
        result.extend(word2[j:])

        return "".join(result)

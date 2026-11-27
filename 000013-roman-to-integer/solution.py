class Solution:
    def romanToInt(self, s: str) -> int:
        values = {"I": 1, "V": 5, "X": 10, "L": 50, "C": 100, "D": 500, "M": 1000}
        result = 0

        for i, ch in enumerate(s):
            curr = values[ch]
            nxt = values[s[i + 1]] if i + 1 < len(s) else 0
            result += -curr if curr < nxt else curr

        return result

class Solution:
    def longestPalindrome(self, s: str) -> str:
        start = 0
        max_len = 1

        def expand(left: int, right: int) -> int:
            while left >= 0 and right < len(s) and s[left] == s[right]:
                left -= 1
                right += 1
            return right - left - 1

        for i in range(len(s)):
            length = max(expand(i, i), expand(i, i + 1))
            if length > max_len:
                max_len = length
                start = i - (length - 1) // 2

        return s[start : start + max_len]

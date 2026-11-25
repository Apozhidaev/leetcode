/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function (s) {
    const lastIndex = new Map();
    let left = 0;
    let maxLen = 0;

    for (let right = 0; right < s.length; right++) {
        const ch = s[right];
        if (lastIndex.has(ch) && lastIndex.get(ch) >= left) {
            left = lastIndex.get(ch) + 1;
        }
        lastIndex.set(ch, right);
        maxLen = Math.max(maxLen, right - left + 1);
    }

    return maxLen;
};

console.log(lengthOfLongestSubstring("abcabcbb"));
console.log(lengthOfLongestSubstring("bbbbb"));
console.log(lengthOfLongestSubstring("pwwkew"));

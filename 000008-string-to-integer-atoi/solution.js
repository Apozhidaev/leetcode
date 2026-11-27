/**
 * @param {string} s
 * @return {number}
 */
var myAtoi = function(s) {
    const INT_MAX = 2 ** 31 - 1;
    const INT_MIN = -(2 ** 31);
    let i = 0;
    const n = s.length;

    while (i < n && s[i] === " ") {
        i++;
    }

    let sign = 1;
    if (i < n && (s[i] === "+" || s[i] === "-")) {
        sign = s[i] === "-" ? -1 : 1;
        i++;
    }

    let result = 0;
    while (i < n && s[i] >= "0" && s[i] <= "9") {
        const digit = s.charCodeAt(i) - 48;

        if (
            result > Math.trunc(INT_MAX / 10) ||
            (result === Math.trunc(INT_MAX / 10) && digit > INT_MAX % 10)
        ) {
            return sign === 1 ? INT_MAX : INT_MIN;
        }

        result = result * 10 + digit;
        i++;
    }

    return sign * result;
};

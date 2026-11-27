/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function(strs) {
    if (strs.length === 0) {
        return "";
    }

    let prefix = strs[0];

    for (let i = 1; i < strs.length; i++) {
        while (!strs[i].startsWith(prefix)) {
            prefix = prefix.slice(0, -1);
            if (prefix === "") {
                return "";
            }
        }
    }

    return prefix;
};

/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix2 = function (strs) {
    if (strs.length === 0) {
      return "";
    }
  
    if (strs.length === 1) {
      return strs[0];
    }
  
    let j = 0;
    const n = Math.min(...strs.map(str => str.length));
  
    while (j < n) {
      const p = strs[0][j];
      for (let i = 1; i < strs.length; i++) {
        const c = strs[i][j];
        if (c !== p) {
          return strs[0].slice(0, j);
        }
      }
      j++;
    }
  
    return strs[0].slice(0, j);
  };

const isPangram = string => {
  if (string.length < 26) return false;
  const pangram = {};
  let parsed = string.toLowerCase().replace(/[^a-z]/gi, '');
  for (let letter of parsed) {
    if (!pangram[letter]) {
      pangram[letter] = 1;
    }
  }
  return Object.keys(pangram).length === 26 ? true : false;
};

module.exports = {
  isPangram
};
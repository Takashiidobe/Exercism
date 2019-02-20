const isPangram = str => {
  str = str.toLowerCase();
  //we need to construct a pangram, which contains one of every letter in the alphabet.
  //we can loop through the entire string, and create a map of every letter in the alphabet, and see if that matches.
  //while we loop through the string, lets loop through.
  const hashmap = {};
  for (let ltr of str) {
    if (!hashmap[ltr]) hashmap[ltr] = 1;
  }
  console.log(Object.keys(hashmap));
  const alphabetstring = 'abcdefghijklmnopqrstuvwxyz';
  const alphabetarray = alphabetstring.split('');
};

export default isPangram;
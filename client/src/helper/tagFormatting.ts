const isPreposition = (word: string): boolean => {
  const prepositions = ['a', 'an', 'the', 'about', 'of', 'at', 'in', 'on', 'to', 'from'];
  return prepositions.includes(word.toLowerCase().trim());
};

const capitalizeFirstLetter = (word: string): string => {
  return word.charAt(0).toUpperCase() + word.slice(1);
};

export const getTagDisplayText = (tag: string): string => {
  let words = tag.split('-').join(' ').split(' ');
  words = words.map((word, index) => {
    if (index === 0 || index === words.length - 1) {
      return capitalizeFirstLetter(word);
    }
    return isPreposition(word) ? word : capitalizeFirstLetter(word)
  });
  return words.join(' ');
};
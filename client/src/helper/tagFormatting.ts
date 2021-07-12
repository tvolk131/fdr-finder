const prepositions = ['a', 'an', 'the', 'about', 'of', 'at', 'in', 'on', 'to', 'from'];

const isPreposition = (word: string): boolean => {
  return prepositions.includes(word.toLowerCase().trim());
};

const capitalizeFirstLetter = (word: string): string => {
  return word.charAt(0).toUpperCase() + word.slice(1);
};

const tagDisplayTextCache: {[key: string]: string} = {};

export const getTagDisplayText = (tag: string): string => {
  if (tagDisplayTextCache[tag] !== undefined) {
    return tagDisplayTextCache[tag];
  }

  let words = tag.split('-').join(' ').split(' ');
  words = words.map((word, index) => {
    if (index === 0 || index === words.length - 1) {
      return capitalizeFirstLetter(word);
    }
    return isPreposition(word) ? word : capitalizeFirstLetter(word)
  });
  const tagDisplayText = words.join(' ');
  tagDisplayTextCache[tag] = tagDisplayText;
  return tagDisplayText;
};
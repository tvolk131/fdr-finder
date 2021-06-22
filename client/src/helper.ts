import {ShowInfo} from './components/showCard';
import {isTrunkNode, LeafDataNode, TrunkDataNode} from './dataNode';

const showInfoToLeafNode = (showInfo: ShowInfo): LeafDataNode => {
  return {
    name: showInfo.title,
    // We're getting the square of the log so that extremely short podcasts are still visible.
    // This is acceptable because the value here is ultimately only meaningful in comparison
    // to other values and *not* as an absolute number.
    value: Math.pow(Math.log(showInfo.lengthInSeconds), 2)
  };
};

export const createTree = (shows: ShowInfo[], levels: {getValue: (show: ShowInfo) => string}[]): TrunkDataNode => {
  const trunk: TrunkDataNode = {
    name: 'Podcasts',
    children: []
  };

  shows.forEach((show) => {
    const showLevels = levels.map((level) => level.getValue(show));
    let currentNode: TrunkDataNode = trunk;
    showLevels.forEach((showLevel) => {
      let foo = currentNode.children.find((child) => child.name === showLevel);
      if (foo === undefined) {
        foo = {name: showLevel, children: []};
        currentNode.children.push(foo);
      }
      if (isTrunkNode(foo)) {
        currentNode = foo;
      }
    });
    currentNode.children.push(showInfoToLeafNode(show));
  });

  return trunk;
};
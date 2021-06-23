import {ShowInfo} from './components/showCard';
import {isTrunkNode, LeafDataNode, TrunkDataNode} from './dataNode';

const showInfoToLeafNode = (showInfo: ShowInfo): LeafDataNode => {
  return {
    name: showInfo.title,
    value: showInfo.lengthInSeconds
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
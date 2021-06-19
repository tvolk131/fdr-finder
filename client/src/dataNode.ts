export interface BaseDataNode {
  name: string
}

export interface TrunkDataNode extends BaseDataNode {
  children: DataNode[]
}

export interface LeafDataNode extends BaseDataNode {
  value: number
}

export type DataNode = TrunkDataNode | LeafDataNode;

export const isTrunkNode = (node: DataNode): node is TrunkDataNode => {
  return 'children' in node;
}

export const isLeafNode = (node: DataNode): node is LeafDataNode => {
  return 'value' in node;
}
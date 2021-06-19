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

export const isTrunkNode = (object: any): object is TrunkDataNode => {
  return 'children' in object;
}

export const isLeafNode = (object: any): object is LeafDataNode => {
  return 'value' in object;
}
import * as React from 'react';
import {useEffect, useRef} from 'react';
import * as d3 from 'd3';
import {TrunkDataNode, LeafDataNode, isLeafNode} from '../dataNode';

const width = 975;
const height = 1200;

const partition = (data: TrunkDataNode | LeafDataNode) => {
  const root = d3.hierarchy(data)
      .sum(d => isLeafNode(d) ? d.value : 0)
      .sort((a, b) => b.height - a.height || ((a.value && b.value) ? b.value - a.value : 0));
  return d3.partition<TrunkDataNode | LeafDataNode>()
      .size([height, (root.height + 1) * width / 3])
    (root);
}
  
const rectHeight = (d: d3.HierarchyRectangularNode<any>) => {
  return d.x1 - d.x0 - Math.min(1, (d.x1 - d.x0) / 2);
}

const labelVisible = (d: d3.HierarchyRectangularNode<any>) => {
  return d.y1 <= width && d.y0 >= 0 && d.x1 - d.x0 > 16;
}

const format = d3.format(',d');

interface ZoomableIcicleProps {
  data: TrunkDataNode
}

// Built from the example at https://observablehq.com/@d3/zoomable-icicle.
export const ZoomableIcicle = (props: ZoomableIcicleProps) => {
  const uniqueId = useRef(Math.floor(Math.random() * 100000000));

  useEffect(() => {
    const color = d3.scaleOrdinal(d3.quantize(d3.interpolateRainbow, props.data.children.length + 1));

    const root = partition(props.data);
    let focus = root;
      
    const svg = d3
      .select(`.target-${uniqueId.current}`)
      .attr('viewBox', [0, 0, width, height].join(', '))
      .style('font', '10px sans-serif');

    const cell = svg.selectAll('g').data(root.descendants()).join('g').attr('transform', (d) => `translate(${d.y0}, ${d.x0})`);

    const rect = cell
      .append('rect')
      .attr('width', d => d.y1 - d.y0 - 1)
      .attr('height', d => rectHeight(d))
      .attr('fill-opacity', 0.6)
      .attr('fill', d => {
        if (!d.depth) {
          return '#ccc';
        }

        while (d.depth > 1 && d.parent) {
          d = d.parent;
        }

        return color(d.data.name);
      })
      .style('cursor', 'pointer')
      .on('click', (event, p) => {
        focus = (focus === p && p.parent) ? p = p.parent : p;
      
        root.each(d => (d as any).target = {
          x0: (d.x0 - p.x0) / (p.x1 - p.x0) * height,
          x1: (d.x1 - p.x0) / (p.x1 - p.x0) * height,
          y0: d.y0 - p.y0,
          y1: d.y1 - p.y0
        });
      
        const t = cell.transition().duration(750)
            .attr('transform', d => `translate(${(d as any).target.y0},${(d as any).target.x0})`);
      
        rect.transition(t).attr('height', d => rectHeight((d as any).target));
        text.transition(t).attr('fill-opacity', d => +labelVisible((d as any).target));
        tspan.transition(t).attr('fill-opacity', d => (labelVisible((d as any).target) ? 1 : 0) * 0.7);
      });

    const text = cell.append('text')
      .style('user-select', 'none')
      .attr('pointer-events', 'none')
      .attr('x', 4)
      .attr('y', 13)
      .attr('fill-opacity', d => +labelVisible(d));

    text.append('tspan')
        .text(d => d.data.name);

    const tspan = text.append('tspan')
        .attr('fill-opacity', d => (labelVisible(d) ? 1 : 0) * 0.7)
        .text(d => ` ${format(d.value!)}`);

    cell.append('title')
        .text(d => `${d.ancestors().map(d => d.data.name).reverse().join('/')}\n${format(d.value!)}`);
  }, [props.data]);

  return (<svg className={`target-${uniqueId.current}`}/>);
}
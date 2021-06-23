import * as React from 'react';
import {useEffect, useRef} from 'react';
import * as d3 from 'd3';
import {DataNode, isLeafNode, TrunkDataNode} from '../dataNode';

const pack = (height: number, width: number, data: DataNode) => {
  const dynPackFn = d3
    .pack<DataNode>()
    .size([width, height])
    .padding(3);

    return dynPackFn(
      d3.hierarchy(data)
        .sum(d => isLeafNode(d) ? d.value : 0)
        .sort((a, b) => (a.value && b.value) ? b.value - a.value : 0)
    );
};

const color = d3.scaleLinear()
                .domain([0, 5])
                .range(['hsl(152,80%,80%)', 'hsl(228,30%,40%)'] as any)
                .interpolate(d3.interpolateHcl as any);

interface ZoomableCirclePackingProps {
  height: number,
  width: number,
  data: TrunkDataNode
}

// Built from the example at https://observablehq.com/@d3/zoomable-circle-packing.
export const ZoomableCirclePacking = (props: ZoomableCirclePackingProps) => {
  const {height, width, data} = props;

  const uniqueId = useRef(Math.floor(Math.random() * 100000000));

  useEffect(() => {
    const root = pack(height, width, data);
    let focus = root;
    let view: [number, number, number];

    const svg = d3
      .select(`.target-${uniqueId.current}`)
      .attr('viewBox', `-${width / 2} -${height / 2} ${width} ${height}`)
      .style('display', 'block')
      .style('background', color(0))
      .style('cursor', 'pointer')
      .on('click', (event) => zoom(event, root));

    // Remove any elements from previous renders.
    svg.selectAll('*').remove();

    const node = svg
      .append('g')
      .selectAll('circle')
      .data(root.descendants().slice(1))
      .join('circle')
      .attr('fill', d => d.children ? color(d.depth) : 'white')
      .attr('pointer-events', d => !d.children ? 'none' : null)
      .on('mouseover', function() { d3.select(this).attr('stroke', '#000'); })
      .on('mouseout', function() { d3.select(this).attr('stroke', null); })
      .on('click', (event, d) => focus !== d && (zoom(event, d), event.stopPropagation()));

    const label = svg
      .append('g')
      .style('font', '10px sans-serif')
      .attr('pointer-events', 'none')
      .attr('text-anchor', 'middle')
      .selectAll('text')
      .data(root.descendants())
      .join('text')
      .style('fill-opacity', d => d.parent === root ? 1 : 0)
      .style('display', d => d.parent === root ? 'inline' : 'none')
      .text(d => d.data.name);

      const zoomTo = (v: [number, number, number]) => {
        const k = width / v[2];

        view = v;

        label.attr('transform', d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
        node.attr('transform', d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
        node.attr('r', d => d.r * k);
      }

      zoomTo([root.x, root.y, root.r * 2]);

      function zoom(event: any, d: d3.HierarchyCircularNode<DataNode>) {
        const focus0 = focus;

        focus = d;

        const transition = svg.transition()
            .duration(event.altKey ? 7500 : 750)
            .tween('zoom', d => {
              const i = d3.interpolateZoom(view, [focus.x, focus.y, focus.r * 2]);
              return t => zoomTo(i(t));
            });

        label
          .filter(function(d) { return d.parent === focus || (this as any).style.display === 'inline'; })
          .transition(transition)
            .style('fill-opacity', d => d.parent === focus ? 1 : 0)
            .on('start', function(d) { if (d.parent === focus) (this as any).style.display = 'inline'; })
            .on('end', function(d) { if (d.parent !== focus) (this as any).style.display = 'none'; });
      }
  }, [height, width, data]);

  return (<svg className={`target-${uniqueId.current}`}/>);
};
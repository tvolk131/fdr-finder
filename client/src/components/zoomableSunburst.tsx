import * as React from 'react';
import {useEffect, useRef} from 'react';
import * as d3 from 'd3';
import {DataNode, isLeafNode, TrunkDataNode} from '../dataNode';

const partition = (data: DataNode) => {
  const root = d3
    .hierarchy(data)
    .sum(d => isLeafNode(d) ? d.value : 0)
    .sort((a, b) => (a.value && b.value) ? b.value - a.value : 0);

  return d3.partition<DataNode>().size([2 * Math.PI, root.height + 1])(root);
};

const arcVisible = (d: any) => {
  return d.y1 <= 3 && d.y0 >= 1 && d.x1 > d.x0;
}

const labelVisible = (d: any) => {
  return d.y1 <= 3 && d.y0 >= 1 && (d.y1 - d.y0) * (d.x1 - d.x0) > 0.03;
}

const format = d3.format(',d');

interface ZoomableSunburstProps {
  width: number,
  data: TrunkDataNode
}

// Built from the example at https://observablehq.com/@d3/zoomable-sunburst.
export const ZoomableSunburst = (props: ZoomableSunburstProps) => {
  const {width, data} = props;
  const radius = width / 6;

  const uniqueId = useRef(Math.floor(Math.random() * 100000000));

  const labelTransform = (d: any) => {
    const x = (d.x0 + d.x1) / 2 * 180 / Math.PI;
    const y = (d.y0 + d.y1) / 2 * radius;
    return `rotate(${x - 90}) translate(${y},0) rotate(${x < 180 ? 0 : 180})`;
  }

  const arc = d3.arc()
    .startAngle((d: any) => d.x0)
    .endAngle((d: any) => d.x1)
    .padAngle((d: any) => Math.min((d.x1 - d.x0) / 2, 0.005))
    .padRadius(radius * 1.5)
    .innerRadius((d: any) => d.y0 * radius)
    .outerRadius((d: any) => Math.max(d.y0 * radius, d.y1 * radius - 1));

  useEffect(() => {
    const root = partition(data);

    root.each((d: any) => d.current = d);

    const svg = d3
      .select(`.target-${uniqueId.current}`)
      .attr('viewBox', `${0} ${0} ${width} ${width}`)
      .style('font', '10px sans-serif');

    // Remove any elements from previous renders.
    svg.selectAll('*').remove();

    const g = svg.append('g')
      .attr('transform', `translate(${width / 2},${width / 2})`);

    const color = d3.scaleOrdinal(d3.quantize(d3.interpolateRainbow, data.children.length + 1));

    const path = g.append('g')
      .selectAll('path')
      .data(root.descendants().slice(1))
      .join('path')
      .attr('fill', d => {
        while (d.depth > 1 && d.parent) {
          d = d.parent;
        }

        return color(d.data.name);
      })
      .attr('fill-opacity', (d: any) => arcVisible(d.current) ? (d.children ? 0.6 : 0.4) : 0)
      .attr('d', (d: any) => arc(d.current));

    path.filter((d: any) => d.children)
      .style('cursor', 'pointer')
      .on('click', clicked);

    path.append('title')
      .text((d: any) => `${d.ancestors().map((d: any) => d.data.name).reverse().join('/')}\n${format(d.value)}`);

    const label = g.append('g')
      .attr('pointer-events', 'none')
      .attr('text-anchor', 'middle')
      .style('user-select', 'none')
      .selectAll('text')
      .data(root.descendants().slice(1))
      .join('text')
      .attr('dy', '0.35em')
      .attr('fill-opacity', (d: any) => +labelVisible(d.current))
      .attr('transform', (d: any) => labelTransform(d.current))
      .text(d => d.data.name);

    function clicked(event: any, p: d3.HierarchyRectangularNode<DataNode>) {
      parent.datum(p.parent || root);

      root.each((d: any) => d.target = {
        x0: Math.max(0, Math.min(1, (d.x0 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
        x1: Math.max(0, Math.min(1, (d.x1 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
        y0: Math.max(0, d.y0 - p.depth),
        y1: Math.max(0, d.y1 - p.depth)
      });

      const t = g.transition().duration(750);

      // Transition the data on all arcs, even the ones that aren’t visible,
      // so that if this transition is interrupted, entering arcs will start
      // the next transition from the desired position.
      path.transition(t as any)
        .tween('data', (d: any) => {
          const i = d3.interpolate(d.current, d.target);
          return (t: any) => d.current = i(t);
        })
        .filter(function(d: any) {
          return (this as any).getAttribute('fill-opacity') || arcVisible(d.target);
        })
        .attr('fill-opacity', (d: any) => arcVisible(d.target) ? (d.children ? 0.6 : 0.4) : 0)
        .attrTween('d', (d: any) => () => arc(d.current) as any);

      label.filter(function(d: any) {
          return (this as any).getAttribute('fill-opacity') || labelVisible(d.target);
      }).transition(t as any)
        .attr('fill-opacity', (d: any) => +labelVisible(d.target))
        .attrTween('transform', (d: any) => () => labelTransform(d.current));
    }

    const parent = g.append('circle')
      .datum(root)
      .attr('r', radius)
      .attr('fill', 'none')
      .attr('pointer-events', 'all')
      .on('click', clicked);
  }, [width, data]);

  return (<svg className={`target-${uniqueId.current}`}/>);
};
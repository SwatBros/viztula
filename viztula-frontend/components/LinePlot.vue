<script setup lang="ts">
import * as d3 from "d3";
import { onMounted, watch } from "vue";

interface Data {
  date: string;
  value: number;
}

interface Props {
  data: Data[];
}

const props = withDefaults(defineProps<Props>(), {
  data: () => [],
})

const renderChart = (data: Data[]) => {
  const width = 800;
  const height = 500;

  const svg = d3.select("svg").attr("width", width).attr("height", height);
  svg.selectAll("*").remove(); // Clear previous content if any
  const g = svg.append("g");

  const parseTime = d3.timeParse("%Y-%m-%dT%H:%M:%S.%L");

  const x = d3.scaleTime().range([0, width]);
  const y = d3.scaleLinear().range([height, 0]);

  const line = d3
    .line<Data>()
    .x((d) => {
      const parsedDate = parseTime(d.date);
      return parsedDate ? x(parsedDate) : 0;
    })
    .y((d) => y(d.value));

  const dateExtent = d3.extent(data, (d) => parseTime(d.date));
  if (dateExtent[0] && dateExtent[1]) {
    x.domain(dateExtent as [Date, Date]);
  }
  y.domain(d3.extent(data, (d) => d.value) as [number, number]);

  g.append("path")
    .datum(data)
    .attr("fill", "none")
    .attr("stroke", "steelblue")
    .attr("stroke-linejoin", "round")
    .attr("stroke-linecap", "round")
    .attr("stroke-width", 1.5)
    .attr("d", line);

  g.append("g")
    .attr("transform", "translate(0," + height + ")")
    .call(d3.axisBottom(x));

  g.append("g").call(d3.axisLeft(y));
};

onMounted(() => {
  renderChart(props.data);
});

watch(() => props.data, (newData) => {
  renderChart(newData);
});
</script>

<template>
  <svg></svg>
</template>

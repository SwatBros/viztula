<script setup lang="ts">
const { data, status, refresh } = await useFetch('/api/hello');

function formatData(data: any) {
  return data.map((d: any) => {
    return {
      date: d.payment_date,
      value: d.sum,
    }
  });
}
</script>

<template>
  <LinePlot :data="formatData(data)" v-if="status === 'success'" />
  <button @click="() => refresh()">Refresh</button>
</template>

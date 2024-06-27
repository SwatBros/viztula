<script setup lang="ts">
const { data } = await useFetch('/api/hello')

function formatData(data: any) {
  return data.map((d: any) => {
    return {
      date: d.payment_date,
      value: parseFloat(d.amount)
    }
  })
}
</script>

<template>
  <LinePlot :data="formatData(data)" />
</template>

<script setup lang="ts">
const { data } = await useFetch('/api/hello')

function formatData(data: any) {
  return data.map((d: any) => {
    return {
      date: d.last_update,
      value: d.actor_id
    }
  })
}
</script>

<template>
  <LinePlot :data="formatData(data)" />
</template>

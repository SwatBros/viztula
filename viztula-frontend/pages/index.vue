<script setup lang="ts">
const { data: promise } = await useFetch('/api/hello')

const data = await promise
</script>

<template>
  <pre>{{ data }}</pre>
</template>

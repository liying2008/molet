<script setup lang="ts">
const props = defineProps<{
  data: string
}>()
</script>

<template>
  <div>
    {{ props.data }}
  </div>
</template>

<style scoped>

</style>

<script setup lang="ts">
import { ContentCopyFilled, DeleteOutlined, FavoriteBorderOutlined } from '@vicons/material'
</script>

<template>
  <div class="header">
    <n-button
      quaternary
      circle
      small
      type="primary"
    >
      <template #icon>
        <n-icon><ContentCopyFilled /></n-icon>
      </template>
    </n-button>
    <n-button
      quaternary
      circle
      small
      type="primary"
    >
      <template #icon>
        <n-icon><FavoriteBorderOutlined /></n-icon>
      </template>
    </n-button>
    <n-button
      quaternary
      circle
      small
      type="error"
    >
      <template #icon>
        <n-icon><DeleteOutlined /></n-icon>
      </template>
    </n-button>
  </div>
</template>

<style scoped lang="scss">
  .header {
    display: flex;
    justify-content: flex-end;
  }
</style>

<script setup lang="ts">
import ItemHeader from './item-header/ItemHeader.vue'
import type { StagingData } from '~/model/staging_data'
import { ContentType } from '~/model/staging_data'

const props = defineProps<{
  stagingData: StagingData
}>()

function getUnicodeContent(arr: number[]) {
  return String.fromCharCode(...arr)
}

function arrayBufferToBase64(arr: number[]) {
  let binary = ''
  const bytes = new Uint8Array(arr)
  const len = bytes.byteLength
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i])
  }
  return window.btoa(binary)
}
</script>

<template>
  <n-card
    :title="stagingData.title"
    class="item-wrapper"
  >
    <template #header-extra>
      <ItemHeader />
    </template>
    <div class="content">
      <div v-if="stagingData.contentType === ContentType.Unicode">
        <pre>{{ getUnicodeContent(stagingData.content) }}</pre>
      </div>
      <div v-else-if="stagingData.contentType === ContentType.Bitmap">
        <img
          :src="`data:image/bmp;base64,${arrayBufferToBase64(stagingData.content)}`"
          :alt="stagingData.title"
        >
      </div>
      <div
        v-else
        class="not-supported"
      >
        [ 暂不支持显示 ]
      </div>
    </div>
  </n-card>
</template>

<style scoped lang="scss">
.item-wrapper {
  margin: 10px;
  border-radius: 10px;

  .content {
    margin-top: 10px;

    pre {
      margin: 0;
      font-family: inherit;
    }

    .not-supported {
      font-size: 12px;
      font-style: italic;
      color: #9b9b9b;
    }
  }
}
</style>

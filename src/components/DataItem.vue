<script setup lang="ts">
import type { StagingData } from '~/model/staging_data'
import { ContentType } from '~/model/staging_data'

const props = defineProps<{
  stagingData: StagingData
}>()

function getUnicodeContent(arr: number[]) {
  return String.fromCharCode(...arr)
}

function getBitmapContent(arr: number[]) {
  const blob = new Blob([new Uint8Array(arr).buffer])
  return URL.createObjectURL(blob)
}
</script>

<template>
  <div class="item-wrapper">
    <div>
      <pre v-if="(stagingData.contentType === ContentType.Unicode)">{{ getUnicodeContent(stagingData.content) }}</pre>
      <div v-if="(stagingData.contentType === ContentType.Bitmap)">
        <img
          :src="getBitmapContent(stagingData.content)"
          alt=""
        >
      </div>
      <div class="menu-icon"></div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.item-wrapper {
  padding: 10px;
  margin: 10px;
  background-color: #ffffff;
  border-radius: 10px;

  pre {
    margin: 0;
    font-family: inherit;
  }
}
</style>

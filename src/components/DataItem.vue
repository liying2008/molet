<script setup lang="ts">
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

function contentToClipboard() {
  console.log(`contentToClipboard: ${props.stagingData.id}`)
}
</script>

<template>
  <div class="item-wrapper">
    <div class="header">
      <img
        class="tool-icon"
        src="https://api.iconify.design/mdi:clipboard-outline.svg"
        alt="Copy"
        @click="contentToClipboard"
      />
      <img
        class="tool-icon"
        src="https://api.iconify.design/mdi:heart-outline.svg"
        alt="Favorite"
      />
      <div class="vertical-divider"></div>
      <img
        class="tool-icon"
        src="https://api.iconify.design/mdi:delete-forever.svg"
        alt="Delete"
      />
    </div>
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

  .header {
    display: flex;
    justify-content: flex-end;
    height: 18px;

    .vertical-divider {
      width: 1px;
      height: 100%;
      margin: 0 4px;
      background-color: #333333;
    }

    .tool-icon {
      margin: 0 4px;
      cursor: pointer;

      :hover {

      }
    }
  }

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

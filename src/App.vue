<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import DataList from './components/DataList.vue'
import Notification from './components/notification/Notification.vue'
</script>

<template>
  <!-- 标题栏 -->
  <div class="container">
    <div
      data-tauri-drag-region
      class="titlebar"
    >
      <div
        id="titlebar-maximize"
        class="titlebar-button"
        @click="appWindow.toggleMaximize()"
      >
        <img
          src="https://api.iconify.design/mdi:window-maximize.svg"
          alt="maximize"
        />
      </div>
      <div
        id="titlebar-close"
        class="titlebar-button"
        @click="appWindow.hide()"
      >
        <img
          src="https://api.iconify.design/mdi:close.svg"
          alt="close"
        />
      </div>
    </div>
    <!-- 应用通知 -->
    <Notification />
    <!-- 数据列表 -->
    <DataList />
  </div>
</template>

<style scoped>
</style>

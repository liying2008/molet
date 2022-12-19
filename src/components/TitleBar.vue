<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
</script>

<template>
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
</template>

<style scoped lang="scss">
.titlebar {
  position: fixed;
  top: 0;
  right: 0;
  left: 0;
  z-index: 9999;
  display: flex;
  justify-content: flex-end;
  height: 30px;
  user-select: none;
  background: #329ea3;

  .titlebar-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;

    &:hover {
      background: #5bbec3;
    }
  }
}
</style>

<script setup lang="ts">
import TitleBar from '~/components/TitleBar.vue'
</script>

<template>
  <n-config-provider>
    <!-- 使用 n-global-style 组件，将主题应用到全局 -->
    <n-global-style />
    <n-dialog-provider>
      <n-message-provider>
        <div
          id="app"
          class="app"
        >
          <!-- 标题栏 -->
          <TitleBar />
          <router-view />
        </div>
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<style scoped lang="scss">

</style>

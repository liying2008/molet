<script lang="ts" setup>
import DataList from '~/components/DataList.vue'
import Notification from '~/components/notification/Notification.vue'
import TitleBar from '~/components/TitleBar.vue'
</script>

<template>
  <div class="layout-wrapper">
    <!-- 标题栏 -->
    <TitleBar />
    <!-- 应用通知 -->
    <Notification />
    <!-- 数据列表 -->
    <DataList />
  </div>
</template>


<style scoped lang="scss">
.layout-wrapper {
}
</style>

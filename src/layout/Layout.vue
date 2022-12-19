<script lang="ts" setup>
import DataList from '~/components/DataList.vue'
import Notification from '~/components/notification/Notification.vue'
</script>

<template>
  <div class="layout-wrapper">
    <!-- 应用通知 -->
    <Notification />
    <!-- 数据列表 -->
    <DataList />
  </div>
</template>


<style scoped lang="scss">
.layout-wrapper {
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding-top: 30px;
  margin: 0;
}
</style>

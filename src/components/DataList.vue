<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import DataItem from './DataItem.vue'
import type { StagingData } from '~/model/staging_data'

const stagingDataList = ref<StagingData[]>([])

onMounted(() => {
  getAllData()
})

async function getAllData() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const time1 = new Date().getTime()
  const dataStr: string = await invoke('get_all_data')
  const time2 = new Date().getTime()
  console.log('invoke cost time', time2 - time1)
  stagingDataList.value = JSON.parse(dataStr)
  const time3 = new Date().getTime()
  console.log('json deserializing cost time', time3 - time2)
  console.log('list', stagingDataList.value)
}
</script>

<template>
  <div class="list-wrapper">
    <!-- <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button> -->
    <DataItem
      v-for="stagingData in stagingDataList"
      :key="stagingData.id!"
      :staging-data="stagingData"
    ></DataItem>
  </div>
</template>

<style scoped lang="scss">
.list-wrapper {

}
</style>

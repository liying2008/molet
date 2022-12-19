<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { Event, UnlistenFn } from '@tauri-apps/api/event'
import { emit, listen } from '@tauri-apps/api/event'
import DataItem from './DataItem.vue'
import type { StagingData } from '~/model/staging_data'

const EVENT_SEND_DATA = 'molet:send-data'

const stagingDataList = ref<StagingData[]>([])
const sendDataUnlisten = ref<UnlistenFn | null>(null)

onMounted(() => {
  getAllData()
  addListeners()
})


async function addListeners() {
  sendDataUnlisten.value = await listen<StagingData[]>(EVENT_SEND_DATA, (event: Event<StagingData[]>) => {
    console.log(`Got error in window ${event.windowLabel}, payload: `, event.payload)
    stagingDataList.value = event.payload
  })
}

async function getAllData() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const time1 = new Date().getTime()
  const data = await invoke<StagingData[]>('get_staging_data')
  const time2 = new Date().getTime()
  console.log('invoke cost time', time2 - time1)
  stagingDataList.value = data
  console.log('list', stagingDataList.value)
}

onUnmounted(() => {
  if (sendDataUnlisten.value) {
    (sendDataUnlisten.value)()
  }
})
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
  padding: 10px;
  background-color: #f1f1f1;
}
</style>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import type { NotificationOption } from './model'

const NOTIFICATION_EVENT_NAME = 'notification'
const unlisten = ref<UnlistenFn | null>(null)

onMounted(() => {
  registerListener()
})

async function registerListener() {
  unlisten.value = await listen<string>(NOTIFICATION_EVENT_NAME, (event) => {
    console.log(`window: ${event.windowLabel}, payload: ${event.payload}`)
    showNotification({ title: event.payload })
  })
}

async function showNotification(option: NotificationOption) {
  let permissionGranted = await isPermissionGranted()
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }
  if (permissionGranted) {
    sendNotification({ title: option.title, body: option.body, icon: option.icon })
  }
}

onUnmounted(() => {
  if (unlisten.value) {
    (unlisten.value)()
  }
})
</script>

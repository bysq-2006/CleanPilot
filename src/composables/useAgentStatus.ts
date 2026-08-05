import { invoke } from '@tauri-apps/api/core'
import { computed, readonly, ref } from 'vue'

const pollIntervalMs = 200
const status = ref('idle')
let pollingTimer: number | undefined

export const syncAgentStatus = async () => {
  try {
    status.value = await invoke<string>('get_agent_status')
  }
  catch {
    status.value = 'idle'
  }
}

export const startAgentStatusPolling = () => {
  if (pollingTimer !== undefined) return

  void syncAgentStatus()
  pollingTimer = window.setInterval(() => {
    void syncAgentStatus()
  }, pollIntervalMs)
}

export const useAgentStatus = () => ({
  status: readonly(status),
  isWorking: computed(() => status.value === 'chatting'),
})

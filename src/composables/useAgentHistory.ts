import { invoke } from '@tauri-apps/api/core'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

export type AgentMessageRole = 'system' | 'user' | 'assistant' | 'tool'

export interface AgentMessage {
  role: AgentMessageRole
  content: string
}

const messages = ref<AgentMessage[]>([])
const isPolling = ref(false)
const syncError = ref<string | null>(null)

let pollingTimer: number | null = null
let activeConsumers = 0
let isSyncing = false

const pollIntervalMs = 1200

async function syncHistory() {
  if (isSyncing) return

  isSyncing = true

  try {
    const history = await invoke<AgentMessage[]>('get_history', {
      startIndex: messages.value.length,
    })

    if (history.length > 0) {
      messages.value = [...messages.value, ...history]
    }

    syncError.value = null
  }
  catch (error) {
    const message = error instanceof Error ? error.message : String(error)
    syncError.value = message
  }
  finally {
    isSyncing = false
  }
}

function clearPollingTimer() {
  if (pollingTimer !== null) {
    window.clearInterval(pollingTimer)
    pollingTimer = null
  }
}

function startPolling() {
  if (pollingTimer !== null) return

  isPolling.value = true
  void syncHistory()

  pollingTimer = window.setInterval(() => {
    void syncHistory()
  }, pollIntervalMs)
}

function stopPolling() {
  clearPollingTimer()
  isPolling.value = false
}

export function useAgentHistory() {
  onMounted(() => {
    activeConsumers += 1

    if (activeConsumers === 1) {
      startPolling()
    }
  })

  onBeforeUnmount(() => {
    activeConsumers = Math.max(0, activeConsumers - 1)

    if (activeConsumers === 0) {
      stopPolling()
    }
  })

  return {
    messages: computed(() => messages.value),
    isPolling: computed(() => isPolling.value),
    syncError: computed(() => syncError.value),
    syncHistory,
  }
}

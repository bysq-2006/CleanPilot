<template>
  <div class="chat-records-view">
    <div class="records-panel">
      <div class="records-header">聊天记录</div>

      <div v-if="loading" class="records-state">加载中...</div>
      <div v-else-if="error" class="records-state records-state-error">{{ error }}</div>
      <div v-else-if="records.length === 0" class="records-state">暂无历史记录</div>

      <div v-else class="records-list">
        <div
          v-for="record in records"
          :key="record.context_id"
          class="record-item"
          role="button"
          tabindex="0"
          @click="handleRestore(record.context_id)"
        >
          <div class="record-preview">{{ record.preview }}</div>
          <div class="record-meta">
            <span class="record-id">{{ record.context_id }}</span>
            <span class="record-count">{{ record.message_count }} 条消息</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import { AgentHistoryStore, type AgentMessage } from '../composables/useAgentHistory'

interface HistoryRecordSummary {
  context_id: string
  scene: string
  message_count: number
  preview: string
  items: AgentMessage[]
}

const router = useRouter()
const agentHistoryStore = new AgentHistoryStore()
const records = ref<HistoryRecordSummary[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

async function loadHistoryRecords() {
  loading.value = true

  try {
    records.value = await invoke<HistoryRecordSummary[]>('list_history_records')
    error.value = null
  }
  catch (err) {
    error.value = err instanceof Error ? err.message : String(err)
  }
  finally {
    loading.value = false
  }
}

async function handleRestore(contextId: string) {
  const record = await invoke<HistoryRecordSummary>('restore_history_context', {
    contextId,
  })

  agentHistoryStore.reset(record.items)

  if (record.scene === 'disk_cleanup') {
    await router.push('/')
  }
}

onMounted(() => {
  void loadHistoryRecords()
})
</script>

<style scoped>
.chat-records-view {
  width: 100%;
  height: 100%;
  padding: 1.25rem;
  box-sizing: border-box;
}

.records-panel {
  width: 100%;
  height: 100%;
  background: #f8fbf9;
  border-radius: 1rem;
  padding: 1rem;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}

.records-header {
  font-size: 1rem;
  font-weight: 600;
  color: #283230;
  margin-bottom: 1rem;
}

.records-state {
  color: #7a8784;
  font-size: 0.875rem;
}

.records-state-error {
  color: #cb4d4d;
}

.records-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  overflow: auto;
}

.record-item {
  padding: 0.875rem 1rem;
  border-radius: 0.875rem;
  background: #ffffff;
  border: 1px solid #e3ece7;
}

.record-preview {
  font-size: 0.9375rem;
  line-height: 1.4;
  color: #1f2a28;
  margin-bottom: 0.5rem;
  word-break: break-word;
}

.record-meta {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  font-size: 0.75rem;
  color: #8a9794;
}

.record-id {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>

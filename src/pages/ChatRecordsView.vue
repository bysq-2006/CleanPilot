<template>
  <div class="chat-records-view">
    <div class="records-panel">
      <div class="records-header">聊天记录</div>

      <div v-if="loading" class="records-state">加载中...</div>
      <div v-else-if="error" class="records-state records-state-error">{{ error }}</div>
      <div v-else-if="records.length === 0" class="records-state">暂无历史记录</div>

      <div v-else class="records-list">
        <div
          v-for="section in groupedRecords"
          :key="section.title"
          class="record-section"
        >
          <div class="record-section-title-row">
            <span class="record-section-marker" aria-hidden="true"></span>
            <div class="record-section-title">{{ section.title }}</div>
          </div>
          <div class="record-section-list">
            <div
              v-for="record in section.items"
              :key="record.context_id"
              class="record-item"
              role="button"
              tabindex="0"
              @click="handleRestore(record.context_id)"
            >
              <div class="record-content">
                <div class="record-preview">{{ record.preview }}</div>
              </div>
              <div class="record-side">
                <span class="record-time">{{ formatRelativeTime(record.updated_at) }}</span>
                <button
                  type="button"
                  class="record-delete-button"
                  @click.stop="handleDelete(record.context_id)"
                >
                  <img src="/Delete.svg" alt="删除" class="record-delete-icon" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import { AgentHistoryStore, type AgentMessage } from '../composables/useAgentHistory'

interface HistoryRecordSummary {
  context_id: string
  scene: string
  updated_at: number
  message_count: number
  preview: string
  items: AgentMessage[]
}

const router = useRouter()
const agentHistoryStore = new AgentHistoryStore()
const records = ref<HistoryRecordSummary[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const groupedRecords = computed(() => {
  const today: HistoryRecordSummary[] = []
  const yesterday: HistoryRecordSummary[] = []
  const older: HistoryRecordSummary[] = []
  const now = new Date()
  const todayStart = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime()
  const yesterdayStart = todayStart - 24 * 60 * 60 * 1000
  const sevenDaysStart = todayStart - 7 * 24 * 60 * 60 * 1000

  for (const record of records.value) {
    const updatedAtMs = record.updated_at * 1000

    if (updatedAtMs >= todayStart) {
      today.push(record)
    }
    else if (updatedAtMs >= yesterdayStart) {
      yesterday.push(record)
    }
    else if (updatedAtMs >= sevenDaysStart) {
      older.push(record)
    }
    else {
      older.push(record)
    }
  }

  return [
    { title: '今天', items: today },
    { title: '昨天', items: yesterday },
    { title: '7天之前', items: older },
  ].filter(section => section.items.length > 0)
})

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

async function handleDelete(contextId: string) {
  await invoke('delete_history_context', { contextId })
  records.value = records.value.filter(record => record.context_id !== contextId)
}

function formatRelativeTime(timestamp: number) {
  const diffMs = Date.now() - timestamp * 1000
  const minute = 60 * 1000
  const hour = 60 * minute
  const day = 24 * hour

  if (diffMs < hour) {
    const minutes = Math.max(1, Math.floor(diffMs / minute))
    return `${minutes} 分钟前`
  }

  if (diffMs < day) {
    const hours = Math.floor(diffMs / hour)
    return `${hours} 小时前`
  }

  const days = Math.floor(diffMs / day)

  if (days < 7) {
    return `${days} 天前`
  }

  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
  })
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
  gap: 1.25rem;
  overflow: auto;
  padding-right: 0.25rem;
}

.record-section {
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
}

.record-section-title-row {
  position: relative;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  min-height: 1.5rem;
}

.record-section-title-row::after {
  content: '';
  position: absolute;
  left: 0.3125rem;
  top: 1.25rem;
  bottom: -0.625rem;
  width: 1px;
  background: linear-gradient(to bottom, rgba(186, 196, 191, 0.8), rgba(186, 196, 191, 0));
}

.record-section:last-child .record-section-title-row::after {
  bottom: 0;
}

.record-section-marker {
  width: 0.625rem;
  height: 0.625rem;
  border-radius: 999px;
  background: #1f2a28;
  flex-shrink: 0;
}

.record-section-title {
  font-size: 1rem;
  font-weight: 500;
  color: #25302d;
}

.record-section-list {
  display: flex;
  flex-direction: column;
  gap: 0.3125rem;
  margin-left: 1.375rem;
}

.record-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1.25rem;
  min-height: 3.125rem;
  padding: 0.625rem 0.25rem;
  border-radius: 0.75rem;
  background: transparent;
  border: 1px solid transparent;
  transition: background-color 0.18s ease, box-shadow 0.18s ease;
  cursor: pointer;
  box-sizing: border-box;
}

.record-item:hover,
.record-item:focus-visible {
  background: rgba(34, 43, 39, 0.06);
  box-shadow: none;
}

.record-content {
  min-width: 0;
  flex: 1;
}

.record-preview {
  font-size: 0.9375rem;
  line-height: 1.35;
  color: #1f2a28;
  word-break: break-word;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.record-side {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0.5rem;
  flex-shrink: 0;
  width: 6.75rem;
}

.record-time {
  font-size: 0.9375rem;
  color: #7d8b87;
  width: 100%;
  text-align: right;
  white-space: nowrap;
  transition: opacity 0.16s ease;
}

.record-delete-button {
  width: 1.875rem;
  height: 1.875rem;
  border: 0;
  border-radius: 0.625rem;
  background: rgba(42, 48, 46, 0.08);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  opacity: 0;
  visibility: hidden;
  pointer-events: none;
  transform: scale(0.92);
  transition: opacity 0.16s ease, visibility 0.16s ease, transform 0.16s ease, background-color 0.16s ease;
  flex-shrink: 0;
}

.record-item:hover .record-time,
.record-item:focus-visible .record-time {
  opacity: 0;
}

.record-item:hover .record-delete-button,
.record-item:focus-visible .record-delete-button {
  opacity: 1;
  visibility: visible;
  pointer-events: auto;
  transform: scale(1);
}

.record-delete-button:hover {
  background: rgba(42, 48, 46, 0.14);
}

.record-delete-icon {
  width: 1rem;
  height: 1rem;
  display: block;
}
</style>

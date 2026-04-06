<template>
  <div class="chat-records-view">
    <div class="records-panel">
      <div class="records-header-block">
        <div class="records-header">聊天记录</div>
        <div class="records-subtitle">按时间快速回看对话、恢复上下文或清理无效记录。</div>
      </div>

      <div v-if="loading" class="records-state-card">
        <div class="records-state records-state-loading">加载中...</div>
      </div>
      <div v-else-if="error" class="records-state-card records-state-card-error">
        <div class="records-state records-state-error">{{ error }}</div>
      </div>
      <div v-else-if="records.length === 0" class="records-state-card records-state-card-empty">
        <div class="records-empty-icon" aria-hidden="true">🕘</div>
        <div class="records-state-title">暂无历史记录</div>
        <div class="records-state-description">新的会话会在完成后自动出现在这里，方便你随时继续处理。</div>
      </div>

      <div v-else class="records-list">
        <div
          v-for="section in groupedRecords"
          :key="section.title"
          class="record-section"
        >
          <div class="record-section-header">
            <div class="record-section-title-row">
              <span class="record-section-marker" aria-hidden="true"></span>
              <div class="record-section-title">{{ section.title }}</div>
            </div>
            <div class="record-section-count">{{ section.items.length }} 条记录</div>
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
              <div class="record-accent" aria-hidden="true"></div>
              <div class="record-content">
                <div class="record-preview">{{ record.preview }}</div>
                <div class="record-meta-row">
                  <span class="record-badge">{{ formatSceneLabel(record.scene) }}</span>
                  <span class="record-meta-divider"></span>
                  <span class="record-meta-text">{{ record.message_count }} 条消息</span>
                  <span class="record-meta-divider"></span>
                  <span class="record-meta-text">{{ formatRelativeTime(record.updated_at) }}</span>
                </div>
              </div>
              <div class="record-side">
                <span class="record-time record-time-full">{{ formatFullDate(record.updated_at) }}</span>
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
  const withinSevenDays: HistoryRecordSummary[] = []
  const beforeSevenDays: HistoryRecordSummary[] = []
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
      withinSevenDays.push(record)
    }
    else {
      beforeSevenDays.push(record)
    }
  }

  return [
    { title: '今天', items: today },
    { title: '昨天', items: yesterday },
    { title: '7天内', items: withinSevenDays },
    { title: '7天之前', items: beforeSevenDays },
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

function formatSceneLabel(scene: string) {
  if (scene === 'disk_cleanup') {
    return '磁盘清理'
  }

  return '普通会话'
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

function formatFullDate(timestamp: number) {
  return new Date(timestamp * 1000).toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
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
  background: #f9fbfc;
}

.records-panel {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.records-header-block {
  padding: 0.25rem 0 1rem;
}

.records-header {
  font-size: 1.375rem;
  line-height: 1.2;
  font-weight: 700;
  color: #1f2a28;
}

.records-subtitle {
  margin-top: 0.375rem;
  font-size: 0.875rem;
  line-height: 1.45;
  color: #70827b;
}

.records-state-card {
  flex: 1;
  min-height: 0;
  border-radius: 1rem;
  border: 1px dashed rgba(180, 195, 188, 0.9);
  background: rgba(255, 255, 255, 0.68);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 2rem;
}

.records-state-card-error {
  border-style: solid;
  border-color: rgba(223, 142, 142, 0.35);
  background: rgba(255, 241, 241, 0.72);
}

.records-state-card-empty {
  gap: 0.5rem;
}

.records-empty-icon {
  width: 3rem;
  height: 3rem;
  border-radius: 999px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  background: rgba(34, 43, 39, 0.08);
}

.records-state-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #2a3632;
}

.records-state-description {
  max-width: 24rem;
  font-size: 0.875rem;
  line-height: 1.6;
  color: #7a8784;
}

.records-state {
  color: #667570;
  font-size: 0.9375rem;
}

.records-state-error {
  color: #cb4d4d;
}

.records-state-loading {
  position: relative;
  padding-left: 1.5rem;
}

.records-state-loading::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  width: 0.875rem;
  height: 0.875rem;
  margin-top: -0.4375rem;
  border-radius: 999px;
  border: 2px solid rgba(31, 42, 40, 0.16);
  border-top-color: #2e6e5c;
  animation: spin 0.8s linear infinite;
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
  gap: 0.75rem;
}

.record-section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.record-section-title-row {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  min-height: 1.5rem;
}

.record-section-marker {
  width: 0.5rem;
  height: 0.5rem;
  border-radius: 999px;
  background: linear-gradient(180deg, #1f2a28 0%, #4c8b73 100%);
  flex-shrink: 0;
  box-shadow: 0 0 0 0.25rem rgba(76, 139, 115, 0.12);
}

.record-section-title {
   font-size: 1rem;
   font-weight: 600;
   color: #25302d;
}

.record-section-count {
  font-size: 0.8125rem;
  color: #87958f;
}

.record-section-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.record-item {
  display: flex;
  align-items: center;
  gap: 0.875rem;
  padding: 0.75rem;
  border: 1px solid rgba(214, 224, 219, 0.95);
  border-radius: 0.875rem;
  background: #fff;
  box-shadow: 0 4px 12px rgba(95, 120, 110, 0.05);
  transition: transform 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
  cursor: pointer;
}

.record-item:hover,
.record-item:focus-visible {
  border-color: rgba(100, 152, 129, 0.55);
  box-shadow: 0 10px 20px rgba(72, 102, 90, 0.09);
  transform: translateY(-1px);
  outline: none;
}

.record-accent {
  width: 0.25rem;
  align-self: stretch;
  border-radius: 999px;
  background: linear-gradient(180deg, #2c6d5a 0%, #9fd3b8 100%);
  flex-shrink: 0;
}

.record-content {
  min-width: 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.record-preview {
  font-size: 0.9375rem;
  line-height: 1.35;
  font-weight: 600;
  color: #1f2a28;
  word-break: break-word;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.record-meta-row {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  min-width: 0;
  flex-wrap: wrap;
}

.record-badge {
  display: inline-flex;
  align-items: center;
  height: 1.5rem;
  padding: 0 0.5rem;
  border-radius: 999px;
  background: rgba(46, 110, 92, 0.1);
  color: #2e6e5c;
  font-size: 0.75rem;
  font-weight: 600;
}

.record-meta-text {
  font-size: 0.75rem;
  color: #76857f;
}

.record-meta-divider {
  width: 0.25rem;
  height: 0.25rem;
  border-radius: 999px;
  background: #b8c5c0;
}

.record-side {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  justify-content: center;
  gap: 0.35rem;
}

.record-time {
  font-size: 0.8125rem;
  font-weight: 600;
  color: #62716b;
  white-space: nowrap;
}

.record-time-full {
  text-align: right;
}

.record-delete-button {
  width: 1.75rem;
  height: 1.75rem;
  border: 0;
  border-radius: 999px;
  background: rgba(42, 48, 46, 0.06);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background-color 0.16s ease;
  flex-shrink: 0;
}

.record-delete-button:hover {
  background: rgba(203, 77, 77, 0.12);
}

.record-delete-icon {
  width: 1rem;
  height: 1rem;
  display: block;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>

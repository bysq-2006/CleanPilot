<template>
  <div class="storage-box-view">
    <div class="storage-box-header-block">
      <div class="storage-box-header">存储箱</div>
      <div class="storage-box-subtitle">已归档任务以卡片形式集中展示。</div>
    </div>

    <div v-if="loading" class="storage-box-state-card">
      <div class="storage-box-state storage-box-state-loading">加载中...</div>
    </div>
    <div v-else-if="error" class="storage-box-state-card storage-box-state-card-error">
      <div class="storage-box-state storage-box-state-error">{{ error }}</div>
    </div>
    <div v-else-if="records.length === 0" class="storage-box-state-card storage-box-state-card-empty">
      <div class="storage-box-empty-icon" aria-hidden="true">📦</div>
      <div class="storage-box-state-title">存储箱还是空的</div>
    </div>
    <div v-else class="storage-box-list">
      <article
        v-for="record in records"
        :key="record.file_name"
        class="storage-box-card"
        :class="`storage-box-card-${taskTheme(record.task_type).theme}`"
        role="button"
        tabindex="0"
      >
        <div class="storage-box-card-top">
          <span class="storage-box-task-badge">{{ taskTheme(record.task_type).label }}</span>
          <button
            type="button"
            class="storage-box-delete-button"
            @click.stop="handleDelete(record.file_name)"
          >
            <img src="/Delete.svg" alt="删除" class="storage-box-delete-icon" />
          </button>
        </div>

        <div class="storage-box-card-title" :title="record.file_name">
          {{ formatDisplayName(record.file_name) }}
        </div>

        <div class="storage-box-card-time">{{ formatRelativeTime(record.saved_at) }}</div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

interface StorageBoxRecordMeta {
  file_name: string
  saved_at: number
  task_type: string
}

const records = ref<StorageBoxRecordMeta[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const taskTheme = (taskType: string) => taskType === 'disk_cleanup'
  ? { label: '清理', theme: 'cleanup' }
  : { label: taskType || '任务', theme: 'default' }

const formatDisplayName = (fileName: string) => fileName
  .replace(/\.json$/i, '')
  .replace(/[-_]+/g, ' ')

function formatRelativeTime(timestamp: number) {
  const diffMs = Date.now() - timestamp * 1000
  const minute = 60 * 1000
  const hour = 60 * minute
  const day = 24 * hour

  if (diffMs < hour) {
    return `${Math.max(1, Math.floor(diffMs / minute))} 分钟前`
  }

  if (diffMs < day) {
    return `${Math.max(1, Math.floor(diffMs / hour))} 小时前`
  }

  return `${Math.max(1, Math.floor(diffMs / day))} 天前`
}

async function loadStorageBoxRecords() {
  loading.value = true

  try {
    records.value = await invoke<StorageBoxRecordMeta[]>('list_storage_box_record_metas')
    error.value = null
  }
  catch (err) {
    error.value = err instanceof Error ? err.message : String(err)
  }
  finally {
    loading.value = false
  }
}

async function handleDelete(fileName: string) {
  await invoke('trash_storage_box_path', { path: fileName })
  records.value = records.value.filter(record => record.file_name !== fileName)
}

onMounted(() => {
  void loadStorageBoxRecords()
})
</script>

<style scoped>
.storage-box-view {
  width: 100%;
  height: 100%;
  padding: 1.25rem;
  box-sizing: border-box;
  background: #f9fbfc;
  display: flex;
  flex-direction: column;
}

.storage-box-header-block {
  padding: 0.25rem 0 1rem;
}

.storage-box-header {
  font-size: 1.375rem;
  font-weight: 700;
  color: #1f2a28;
}

.storage-box-subtitle {
  margin-top: 0.375rem;
  font-size: 0.875rem;
  color: #70827b;
}

.storage-box-state-card {
  flex: 1;
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

.storage-box-state-card-error {
  border-style: solid;
  border-color: rgba(223, 142, 142, 0.35);
  background: rgba(255, 241, 241, 0.72);
}

.storage-box-state-card-empty {
  gap: 0.5rem;
}

.storage-box-empty-icon {
  width: 3rem;
  height: 3rem;
  border-radius: 999px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  background: rgba(34, 43, 39, 0.08);
}

.storage-box-state-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #2a3632;
}

.storage-box-state,
.storage-box-card-time {
  font-size: 0.8125rem;
  color: #71807a;
}

.storage-box-state-error {
  color: #cb4d4d;
}

.storage-box-state-loading {
  position: relative;
  padding-left: 1.5rem;
}

.storage-box-state-loading::before {
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

.storage-box-list {
  flex: 1;
  min-height: 0;
  overflow: auto;
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 1rem;
  align-content: start;
}

.storage-box-card {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  min-height: 8.75rem;
  min-width: 0;
  padding: 1rem;
  border-radius: 1rem;
  border: 1px solid rgba(214, 224, 219, 0.95);
  background: #fff;
  box-shadow: 0 4px 12px rgba(95, 120, 110, 0.05);
  cursor: pointer;
  transition: transform 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
}

.storage-box-card:hover,
.storage-box-card:focus-visible {
  border-color: rgba(100, 152, 129, 0.4);
  box-shadow: 0 10px 20px rgba(72, 102, 90, 0.08);
  transform: translateY(-1px);
  outline: none;
}

.storage-box-card-cleanup {
  background: linear-gradient(180deg, rgba(241, 250, 245, 0.96) 0%, #ffffff 72%);
}

.storage-box-card-default {
  background: linear-gradient(180deg, rgba(244, 247, 249, 0.96) 0%, #ffffff 72%);
}

.storage-box-card-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
}

.storage-box-task-badge {
  display: inline-flex;
  align-items: center;
  height: 1.625rem;
  padding: 0 0.625rem;
  border-radius: 999px;
  background: rgba(58, 145, 93, 0.12);
  color: #2f7a4a;
  font-size: 0.75rem;
  font-weight: 700;
}

.storage-box-card-default .storage-box-task-badge {
  background: rgba(83, 108, 129, 0.12);
  color: #4f6477;
}

.storage-box-delete-button {
  width: 1.9rem;
  height: 1.9rem;
  border: 0;
  border-radius: 999px;
  background: rgba(42, 48, 46, 0.06);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
}

.storage-box-delete-button:hover {
  background: rgba(203, 77, 77, 0.12);
}

.storage-box-delete-icon {
  width: 1rem;
  height: 1rem;
  display: block;
}

.storage-box-card-title {
  font-size: 0.9375rem;
  line-height: 1.35;
  font-weight: 700;
  color: #21302c;
  word-break: break-word;
  min-width: 0;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.storage-box-card-time {
  margin-top: auto;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>

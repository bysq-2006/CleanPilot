<template>
  <div class="list-directory-message">
    <CommonFrameBox max-height="16rem">
      <div class="header-row">
        <span class="title">目录扫描</span>
        <span v-if="directoryPath" class="path">{{ directoryPath }}</span>
      </div>

      <div v-if="entries.length" class="entry-list">
        <div v-for="(entry, index) in entries" :key="`${entry.path}-${index}`" class="entry-item">
          <div class="entry-main">
            <span class="entry-name" :title="entry.name">{{ entry.name }}</span>
            <span class="entry-type" :class="entry.type === '目录' ? 'dir' : 'file'">{{ entry.type }}</span>
          </div>
          <div class="entry-meta">
            <span>{{ formatBytes(entry.sizeBytes) }}</span>
            <span class="entry-path" :title="entry.path">{{ entry.path }}</span>
          </div>
        </div>
      </div>

      <pre v-else class="raw-content">{{ displayContent }}</pre>
    </CommonFrameBox>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'
import CommonFrameBox from '../../../CommonFrameBox.vue'

interface DirectoryEntry {
  name: string
  type: '目录' | '文件'
  sizeBytes: number
  path: string
}

const props = defineProps<{
  message: AgentMessage
}>()

const displayContent = computed(() => (props.message.content ?? '').trim())

const directoryPath = computed(() => {
  const matched = displayContent.value.match(/目录:\s*(.+)/)
  return matched?.[1]?.trim() ?? ''
})

const entries = computed<DirectoryEntry[]>(() => {
  const lines = displayContent.value.split('\n')

  return lines
    .filter((line) => line.startsWith('- 名称: '))
    .map((line) => {
      const matched = line.match(/^- 名称: (.+?) \| 类型: (目录|文件) \| 大小: (\d+) 字节 \| 路径: (.+)$/)
      if (!matched) return null

      return {
        name: matched[1],
        type: matched[2] as '目录' | '文件',
        sizeBytes: Number(matched[3]),
        path: matched[4],
      }
    })
    .filter((entry): entry is DirectoryEntry => entry !== null)
})

const formatBytes = (bytes: number) => {
  if (!Number.isFinite(bytes)) return '未知大小'
  if (bytes < 1024) return `${bytes} B`

  const units = ['KB', 'MB', 'GB', 'TB']
  let value = bytes / 1024
  let unitIndex = 0

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024
    unitIndex += 1
  }

  return `${value.toFixed(value >= 10 ? 0 : 1)} ${units[unitIndex]}`
}
</script>

<style scoped>
.list-directory-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
}

.header-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem 0.75rem;
  margin-bottom: 0.625rem;
}

.title {
  font-size: 0.8125rem;
  font-weight: 700;
  color: #334155;
}

.path {
  font-size: 0.75rem;
  color: #64748b;
  word-break: break-all;
}

.entry-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.entry-item {
  padding: 0.625rem 0.6875rem;
  border-radius: 0.625rem;
  border: 0.0625rem solid #e2e8f0;
  background: #f8fafc;
}

.entry-main {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.entry-name {
  font-size: 0.875rem;
  color: #0f172a;
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.entry-type {
  flex-shrink: 0;
  font-size: 0.6875rem;
  line-height: 1;
  padding: 0.2rem 0.35rem;
  border-radius: 999px;
  border: 0.0625rem solid #cbd5e1;
  color: #475569;
  background: #ffffff;
}

.entry-type.dir {
  color: #1d4ed8;
  border-color: #bfdbfe;
  background: #eff6ff;
}

.entry-meta {
  margin-top: 0.35rem;
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem 0.625rem;
  font-size: 0.75rem;
  color: #64748b;
}

.entry-path {
  word-break: break-all;
}

.raw-content {
  margin: 0;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.8125rem;
}
</style>

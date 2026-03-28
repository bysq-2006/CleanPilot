<template>
  <div class="find-large-entries-message">
    <CommonFrameBox max-height="18rem">
      <div v-if="isScanning" class="scanning-state">
        <span class="spinner" aria-hidden="true" />
        <span class="scanning-text">正在扫描中，请稍候…</span>
      </div>

      <template v-else>
      <div class="header-row">
        <span class="title">空间占用排行榜</span>
        <span v-if="scanPath" class="path">{{ scanPath }}</span>
      </div>

      <div v-if="rankedEntries.length" class="rank-list">
        <div v-for="(entry, index) in rankedEntries" :key="`${entry.path}-${index}`" class="rank-item">
          <div class="rank-left">
            <span class="rank-index">#{{ index + 1 }}</span>
            <span class="rank-name" :title="entry.path">{{ entry.path }}</span>
            <span class="rank-type" :class="entry.type === '文件夹' ? 'dir' : 'file'">{{ entry.type }}</span>
          </div>

          <div class="rank-right">
            <div class="bar-track">
              <div class="bar-fill" :style="{ width: `${entry.percent}%` }" />
            </div>
            <span class="size-label">{{ formatMb(entry.sizeBytes) }} MB · {{ formatPercent(entry.percent) }}</span>
          </div>
        </div>
      </div>

      <pre v-else class="raw-content">{{ displayContent }}</pre>
      </template>
    </CommonFrameBox>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'
import CommonFrameBox from '../../../CommonFrameBox.vue'

interface LargeEntryRaw {
  type: '文件' | '文件夹'
  sizeBytes: number
  path: string
}

interface LargeEntryRanked extends LargeEntryRaw {
  percent: number
}

const props = defineProps<{
  message: AgentMessage
}>()

const isScanning = computed(() => props.message.ready === true)

const displayContent = computed(() => (props.message.content ?? '').trim())

const scanPath = computed(() => {
  const matched = displayContent.value.match(/扫描目录:\s*(.+)/)
  return matched?.[1]?.trim() ?? ''
})

const rawEntries = computed<LargeEntryRaw[]>(() => {
  const lines = displayContent.value.split('\n')

  return lines
    .map((line) => {
      const matched = line.match(/^- 类型: (文件|文件夹) \| 大小: (\d+) 字节 \| 路径: (.+)$/)
      if (!matched) return null

      return {
        type: matched[1] as '文件' | '文件夹',
        sizeBytes: Number(matched[2]),
        path: matched[3],
      }
    })
    .filter((entry): entry is LargeEntryRaw => entry !== null)
})

const totalSizeBytes = computed(() => rawEntries.value.reduce((sum, entry) => sum + entry.sizeBytes, 0))

const rankedEntries = computed<LargeEntryRanked[]>(() => {
  if (!rawEntries.value.length) return []

  const sorted = [...rawEntries.value].sort((a, b) => b.sizeBytes - a.sizeBytes)

  return sorted.map((entry) => ({
    ...entry,
    percent: totalSizeBytes.value > 0 ? (entry.sizeBytes / totalSizeBytes.value) * 100 : 0,
  }))
})

const formatMb = (bytes: number) => (bytes / (1024 * 1024)).toFixed(1)
const formatPercent = (percent: number) => `${percent.toFixed(1)}%`
</script>

<style scoped>
.find-large-entries-message {
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

.scanning-state {
  min-height: 5.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.625rem;
  color: #334155;
}

.spinner {
  width: 1rem;
  height: 1rem;
  border-radius: 999px;
  border: 0.125rem solid #cbd5e1;
  border-top-color: #334155;
  animation: spin 0.85s linear infinite;
}

.scanning-text {
  font-size: 0.8125rem;
  color: #475569;
}

.title {
  font-size: 0.8125rem;
  font-weight: 700;
  color: #0f172a;
}

.path {
  font-size: 0.75rem;
  color: #64748b;
  word-break: break-all;
}

.rank-list {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.rank-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.5rem 0.625rem;
  border-radius: 0.625rem;
  background: #f8fafc;
  border: 0.0625rem solid #e2e8f0;
}

.rank-left {
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 0.45rem;
  flex: 1;
}

.rank-index {
  flex-shrink: 0;
  font-size: 0.75rem;
  color: #475569;
  font-weight: 700;
}

.rank-name {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.8125rem;
  color: #0f172a;
}

.rank-type {
  flex-shrink: 0;
  font-size: 0.6875rem;
  line-height: 1;
  padding: 0.2rem 0.35rem;
  border-radius: 999px;
  border: 0.0625rem solid #cbd5e1;
  color: #475569;
  background: #ffffff;
}

.rank-type.dir {
  color: #2563eb;
  border-color: #bfdbfe;
  background: #eff6ff;
}

.rank-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  width: min(16rem, 45%);
  flex-shrink: 0;
}

.bar-track {
  flex: 1;
  height: 0.5rem;
  border-radius: 999px;
  overflow: hidden;
  background: #e2e8f0;
}

.bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #475569 0%, #334155 100%);
}

.size-label {
  flex-shrink: 0;
  font-size: 0.75rem;
  color: #334155;
  font-variant-numeric: tabular-nums;
}

.raw-content {
  margin: 0;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.8125rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>

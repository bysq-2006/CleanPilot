<!-- 展示高占用进程列表工具的调用结果。 -->
<template>
  <div class="list-processes-message">
    <CommonFrameBox max-height="18rem">
      <div v-if="isLoading" class="loading-state">
        <span class="spinner" aria-hidden="true" />
        <span>正在采样进程占用…</span>
      </div>

      <template v-else>
        <div class="header-row">
          <span class="title">进程占用</span>
          <span class="meta">{{ sortLabel }} · {{ processes.length }}</span>
        </div>

        <div v-if="processes.length" class="process-table">
          <div class="col-head" aria-hidden="true">
            <span class="col-name">进程</span>
            <span class="col-bar">占用</span>
            <span class="col-cpu">CPU</span>
            <span class="col-mem">内存</span>
          </div>

          <div
            v-for="(item, index) in processes"
            :key="`${item.pid}-${index}`"
            class="process-row"
            :title="item.path"
          >
            <span class="rank">{{ String(index + 1).padStart(2, '0') }}</span>
            <span class="name-cell">
              <span class="name">{{ item.name }}</span>
              <span v-if="item.category !== '普通'" class="mark" :class="categoryClass(item.category)">{{ item.category }}</span>
            </span>
            <span class="bar-cell">
              <span class="bar-track">
                <span class="bar-fill" :style="{ width: `${item.barPercent}%` }" />
              </span>
            </span>
            <span class="num cpu">{{ item.cpu.toFixed(1) }}%</span>
            <span class="num mem">{{ formatBytes(item.memory) }}</span>
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

interface ProcessRow {
  name: string
  pid: number
  cpu: number
  memory: number
  disk: number
  category: string
  path: string
  barPercent: number
}

const props = defineProps<{
  message: AgentMessage
}>()

const isLoading = computed(() => props.message.ready === true)
const displayContent = computed(() => (props.message.content ?? '').trim())

const sortBy = computed(() => displayContent.value.match(/^排序:\s*(.+)$/m)?.[1]?.trim() ?? 'cpu')

const sortLabel = computed(() => {
  if (sortBy.value === 'memory') return '按内存'
  if (sortBy.value === 'disk') return '按磁盘'
  return '按 CPU'
})

const processes = computed<ProcessRow[]>(() => {
  const rows = displayContent.value
    .split('\n')
    .map((line) => {
      const matched = line.match(/^- 名称: (.+?) \| PID: (\d+) \| CPU: ([\d.]+)% \| 内存: (\d+) 字节 \| 磁盘读取: (\d+) 字节 \| 磁盘写入: (\d+) 字节 \| 类别: (.+?) \| 路径: (.+)$/)
      if (!matched) return null
      return {
        name: matched[1],
        pid: Number(matched[2]),
        cpu: Number(matched[3]),
        memory: Number(matched[4]),
        disk: Number(matched[5]) + Number(matched[6]),
        category: matched[7],
        path: matched[8],
      }
    })
    .filter((item): item is Omit<ProcessRow, 'barPercent'> => item !== null)

  const metric = (item: Omit<ProcessRow, 'barPercent'>) => {
    if (sortBy.value === 'memory') return item.memory
    if (sortBy.value === 'disk') return item.disk
    return item.cpu
  }
  const max = Math.max(...rows.map(metric), 0.0001)

  return rows.map(item => ({
    ...item,
    barPercent: Math.max(4, Math.round((metric(item) / max) * 100)),
  }))
})

const categoryClass = (category: string) => {
  if (category === '系统关键') return 'critical'
  if (category === '安全软件') return 'security'
  return 'normal'
}

const formatBytes = (bytes: number) => {
  if (!Number.isFinite(bytes)) return '—'
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
.list-processes-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
}

.loading-state {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #64748b;
  font-size: 0.8125rem;
}

.spinner {
  width: 0.875rem;
  height: 0.875rem;
  border-radius: 999px;
  border: 2px solid rgba(100, 116, 139, 0.2);
  border-top-color: #475569;
  animation: spin 0.8s linear infinite;
}

.header-row {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0 0.15rem 0.45rem;
  border-bottom: 0.0625rem solid #e8eef3;
}

.title {
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: #64748b;
}

.meta {
  font-size: 0.6875rem;
  color: #94a3b8;
}

.process-table {
  display: flex;
  flex-direction: column;
}

.col-head,
.process-row {
  display: grid;
  grid-template-columns: 1.5rem minmax(0, 1.3fr) minmax(4.5rem, 1fr) 3.4rem 4.2rem;
  align-items: center;
  column-gap: 0.55rem;
}

.col-head {
  padding: 0.35rem 0.15rem 0.2rem;
  font-size: 0.625rem;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: #94a3b8;
}

.col-name {
  grid-column: 2;
}

.col-bar {
  text-align: left;
}

.col-cpu,
.col-mem {
  text-align: right;
}

.process-row {
  min-height: 1.75rem;
  padding: 0.2rem 0.15rem;
  border-bottom: 0.0625rem dashed #eef2f6;
}

.process-row:last-child {
  border-bottom: 0;
}

.rank {
  font-size: 0.6875rem;
  font-variant-numeric: tabular-nums;
  color: #cbd5e1;
}

.name-cell {
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 0.35rem;
}

.name {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.8125rem;
  font-weight: 600;
  color: #1e293b;
}

.mark {
  flex-shrink: 0;
  font-size: 0.625rem;
  color: #94a3b8;
}

.mark.critical {
  color: #b45309;
}

.mark.security {
  color: #2563eb;
}

.bar-track {
  display: block;
  height: 0.28rem;
  border-radius: 999px;
  overflow: hidden;
  background: #eef2f6;
}

.bar-fill {
  display: block;
  height: 100%;
  border-radius: inherit;
  background: #64748b;
}

.num {
  font-size: 0.75rem;
  font-variant-numeric: tabular-nums;
  text-align: right;
  color: #475569;
}

.cpu {
  font-weight: 600;
  color: #334155;
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

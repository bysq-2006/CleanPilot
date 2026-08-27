<!-- 展示系统整体运行情况工具的调用结果。 -->
<template>
  <div class="system-perf-message">
    <CommonFrameBox max-height="16rem">
      <div v-if="isLoading" class="loading-state">
        <span class="spinner" aria-hidden="true" />
        <span>正在读取系统状态…</span>
      </div>

      <template v-else-if="parsed.valid">
        <div class="header-row">
          <span class="title">系统状态</span>
          <span class="meta">{{ parsed.os }} · {{ parsed.deviceKind }}</span>
        </div>

        <div class="metric">
          <div class="metric-label">
            <span>CPU</span>
            <span>{{ parsed.cpuUsage.toFixed(1) }}%</span>
          </div>
          <div class="bar-track">
            <div class="bar-fill" :style="{ width: `${clampPercent(parsed.cpuUsage)}%` }" />
          </div>
        </div>

        <div class="metric">
          <div class="metric-label">
            <span>内存</span>
            <span>{{ formatBytes(parsed.usedMemory) }} / {{ formatBytes(parsed.totalMemory) }}</span>
          </div>
          <div class="bar-track">
            <div class="bar-fill" :style="{ width: `${memoryPercent}%` }" />
          </div>
        </div>

        <div class="power-plan">电源计划：{{ parsed.powerPlan }}</div>

        <div v-if="parsed.disks.length" class="disk-list">
          <div v-for="disk in parsed.disks" :key="disk.mountPoint" class="disk-item">
            <span class="disk-name">{{ disk.mountPoint }}</span>
            <span class="disk-size">可用 {{ formatBytes(disk.available) }} / {{ formatBytes(disk.total) }}</span>
          </div>
        </div>
      </template>

      <pre v-else class="raw-content">{{ displayContent }}</pre>
    </CommonFrameBox>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'
import CommonFrameBox from '../../../CommonFrameBox.vue'

interface DiskRow {
  mountPoint: string
  total: number
  used: number
  available: number
}

const props = defineProps<{
  message: AgentMessage
}>()

const isLoading = computed(() => props.message.ready === true)
const displayContent = computed(() => (props.message.content ?? '').trim())

const parsed = computed(() => {
  const content = displayContent.value
  const os = content.match(/^系统:\s*(.+)$/m)?.[1]?.trim() ?? ''
  const deviceKind = content.match(/^设备类型:\s*(.+)$/m)?.[1]?.trim() ?? ''
  const powerPlan = content.match(/^电源计划:\s*(.+)$/m)?.[1]?.trim() ?? ''
  const cpuUsage = Number(content.match(/^CPU 占用:\s*([\d.]+)%/m)?.[1] ?? NaN)
  const totalMemory = Number(content.match(/^内存总量:\s*(\d+)\s*字节/m)?.[1] ?? NaN)
  const usedMemory = Number(content.match(/^内存已用:\s*(\d+)\s*字节/m)?.[1] ?? NaN)

  const disks: DiskRow[] = content
    .split('\n')
    .map((line) => {
      const matched = line.match(/^- 挂载点: (.+?) \| 文件系统: .+? \| 总容量: (\d+) 字节 \| 已用: (\d+) 字节 \| 可用: (\d+) 字节$/)
      if (!matched) return null
      return {
        mountPoint: matched[1],
        total: Number(matched[2]),
        used: Number(matched[3]),
        available: Number(matched[4]),
      }
    })
    .filter((item): item is DiskRow => item !== null)

  return {
    valid: Number.isFinite(cpuUsage) && Number.isFinite(totalMemory) && totalMemory > 0,
    os,
    deviceKind,
    powerPlan,
    cpuUsage,
    totalMemory,
    usedMemory,
    disks,
  }
})

const memoryPercent = computed(() => {
  if (!parsed.value.valid || parsed.value.totalMemory <= 0) return 0
  return clampPercent((parsed.value.usedMemory / parsed.value.totalMemory) * 100)
})

const clampPercent = (value: number) => Math.max(0, Math.min(100, Math.round(value)))

const formatBytes = (bytes: number) => {
  if (!Number.isFinite(bytes)) return '未知'
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
.system-perf-message {
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
  flex-wrap: wrap;
  align-items: baseline;
  gap: 0.5rem 0.75rem;
  margin-bottom: 0.75rem;
}

.title {
  font-size: 0.875rem;
  font-weight: 700;
  color: #0f172a;
}

.meta {
  font-size: 0.75rem;
  color: #64748b;
}

.metric + .metric {
  margin-top: 0.625rem;
}

.metric-label {
  display: flex;
  justify-content: space-between;
  gap: 0.75rem;
  font-size: 0.75rem;
  color: #475569;
  margin-bottom: 0.25rem;
}

.bar-track {
  height: 0.5rem;
  border-radius: 999px;
  overflow: hidden;
  background: #f1f5f9;
  border: 0.0625rem solid #e2e8f0;
}

.bar-fill {
  height: 100%;
  background: #64748b;
}

.power-plan {
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: #475569;
}

.disk-list {
  margin-top: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.disk-item {
  display: flex;
  justify-content: space-between;
  gap: 0.75rem;
  font-size: 0.75rem;
  color: #64748b;
}

.disk-name {
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

<template>
  <div class="disk-info-message">
    <CommonFrameBox max-height="14rem">
      <template v-if="parsed.valid">
        <div class="disk-title">当前磁盘：{{ parsed.mountPoint || '未知磁盘' }}</div>

        <div class="usage-bar" role="img" :aria-label="`磁盘使用情况：可用 ${availablePercent}%`">
          <div class="used-segment" :style="{ width: `${usedPercent}%` }" />
          <div class="available-segment" :style="{ width: `${availablePercent}%` }" />
        </div>

        <div class="usage-meta">
          <span>可用 {{ availablePercent }}%（{{ formatGb(parsed.availableSpace) }} GB）</span>
          <span>已用 {{ usedPercent }}%（{{ formatGb(usedSpace) }} GB）</span>
        </div>

        <div class="total-meta">总容量 {{ formatGb(parsed.totalSpace) }} GB</div>
      </template>

      <pre v-else class="raw-content">{{ displayContent }}</pre>
    </CommonFrameBox>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'
import CommonFrameBox from '../../../CommonFrameBox.vue'

const props = defineProps<{
  message: AgentMessage
}>()

const displayContent = computed(() => (props.message.content ?? '').trim())

const parsed = computed(() => {
  const content = displayContent.value

  const mountPoint = content.match(/挂载点:\s*(.+)/)?.[1]?.trim() ?? ''
  const totalSpace = Number(content.match(/总容量:\s*(\d+)\s*字节/)?.[1] ?? NaN)
  const availableSpace = Number(content.match(/可用空间:\s*(\d+)\s*字节/)?.[1] ?? NaN)

  const valid = Number.isFinite(totalSpace)
    && Number.isFinite(availableSpace)
    && totalSpace > 0

  return {
    valid,
    mountPoint,
    totalSpace,
    availableSpace,
  }
})

const availablePercent = computed(() => {
  if (!parsed.value.valid) return 0
  const percent = (parsed.value.availableSpace / parsed.value.totalSpace) * 100
  return Math.max(0, Math.min(100, Math.round(percent)))
})

const usedPercent = computed(() => 100 - availablePercent.value)

const usedSpace = computed(() => {
  if (!parsed.value.valid) return 0
  return Math.max(0, parsed.value.totalSpace - parsed.value.availableSpace)
})

const formatGb = (bytes: number) => (bytes / (1024 * 1024 * 1024)).toFixed(2)
</script>

<style scoped>
.disk-info-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
}

.disk-title {
  font-size: 0.875rem;
  font-weight: 700;
  color: #0f172a;
  margin-bottom: 0.625rem;
}

.usage-bar {
  width: 100%;
  height: 0.875rem;
  border-radius: 999px;
  overflow: hidden;
  display: flex;
  border: 0.0625rem solid #e2e8f0;
  background: #f1f5f9;
}

.used-segment {
  background: #9ca3af;
}

.available-segment {
  background: #4db071;
}

.usage-meta {
  margin-top: 0.5rem;
  display: flex;
  justify-content: space-between;
  gap: 0.75rem;
  font-size: 0.75rem;
  color: #475569;
}

.total-meta {
  margin-top: 0.375rem;
  font-size: 0.75rem;
  color: #64748b;
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


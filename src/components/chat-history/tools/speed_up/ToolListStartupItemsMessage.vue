<!-- 展示开机启动项工具的调用结果。 -->
<template>
  <div class="list-startup-message">
    <CommonFrameBox max-height="18rem">
      <div v-if="isLoading" class="loading-state">
        <span class="spinner" aria-hidden="true" />
        <span>正在读取开机启动项…</span>
      </div>

      <template v-else>
        <div class="header-row">
          <span class="title">开机启动</span>
          <span class="meta">
            <span class="count-on">{{ enabledCount }} 启用</span>
            <span class="count-off">{{ disabledCount }} 禁用</span>
          </span>
        </div>

        <div v-if="grouped.length" class="startup-ledger">
          <template v-for="group in grouped" :key="group.status">
            <div class="group-label">{{ group.status }}</div>
            <div
              v-for="(item, index) in group.items"
              :key="`${item.name}-${index}`"
              class="startup-row"
              :class="{ off: item.status === '已禁用' }"
              :title="item.command"
            >
              <span class="pip" :class="item.status === '已禁用' ? 'pip-off' : 'pip-on'" />
              <span class="name">{{ item.name }}</span>
              <span class="source">{{ formatSource(item.source) }}</span>
            </div>
          </template>
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

interface StartupRow {
  name: string
  status: string
  source: string
  command: string
}

const props = defineProps<{
  message: AgentMessage
}>()

const isLoading = computed(() => props.message.ready === true)
const displayContent = computed(() => (props.message.content ?? '').trim())

const items = computed<StartupRow[]>(() => {
  return displayContent.value
    .split('\n')
    .map((line) => {
      const matched = line.match(/^- 名称: (.+?) \| 状态: (.+?) \| (?:位置: .+? \| )?来源: (.+?) \| 命令: (.+)$/)
      if (!matched) return null
      return {
        name: matched[1],
        status: matched[2],
        source: matched[3],
        command: matched[4],
      }
    })
    .filter((item): item is StartupRow => item !== null)
})

const enabledCount = computed(() => items.value.filter(item => item.status === '已启用').length)
const disabledCount = computed(() => items.value.filter(item => item.status === '已禁用').length)

const grouped = computed(() => {
  const order = ['已启用', '未知', '已禁用']
  return order
    .map(status => ({
      status,
      items: items.value.filter(item => item.status === status),
    }))
    .filter(group => group.items.length > 0)
})

const formatSource = (source: string) => {
  if (source.includes('HKCU') && source.includes('RunOnce')) return 'HKCU · RunOnce'
  if (source.includes('HKLM') && source.includes('WOW6432Node') && source.includes('RunOnce')) return 'HKLM32 · RunOnce'
  if (source.includes('HKLM') && source.includes('WOW6432Node')) return 'HKLM32 · Run'
  if (source.includes('HKCU') && source.includes('Run')) return 'HKCU · Run'
  if (source.includes('HKLM') && source.includes('RunOnce')) return 'HKLM · RunOnce'
  if (source.includes('HKLM')) return 'HKLM · Run'
  if (source.includes('用户启动')) return '用户启动文件夹'
  if (source.includes('公共启动')) return '公共启动文件夹'
  return source
}
</script>

<style scoped>
.list-startup-message {
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
  display: flex;
  gap: 0.625rem;
  font-size: 0.6875rem;
  font-variant-numeric: tabular-nums;
}

.count-on {
  color: #b45309;
}

.count-off {
  color: #94a3b8;
}

.startup-ledger {
  display: flex;
  flex-direction: column;
}

.group-label {
  padding: 0.5rem 0.15rem 0.2rem;
  font-size: 0.625rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #94a3b8;
}

.startup-row {
  display: grid;
  grid-template-columns: 0.6rem minmax(0, 1fr) auto;
  align-items: center;
  column-gap: 0.55rem;
  min-height: 1.7rem;
  padding: 0.12rem 0.15rem;
  border-bottom: 0.0625rem dashed #eef2f6;
}

.startup-row:last-child {
  border-bottom: 0;
}

.startup-row.off .name {
  color: #94a3b8;
  font-weight: 500;
}

.pip {
  width: 0.375rem;
  height: 0.375rem;
  border-radius: 999px;
}

.pip-on {
  background: #d97706;
}

.pip-off {
  background: #cbd5e1;
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

.source {
  font-size: 0.6875rem;
  color: #94a3b8;
  white-space: nowrap;
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

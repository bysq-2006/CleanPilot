<!-- 展示电脑变快建议写入任务的工具消息。 -->
<template>
  <div class="write-checklist-message">
    <div class="checklist-card" :class="{ 'is-error': !isSuccess }">
      <div class="accent-strip" />

      <div class="card-body">
        <div class="card-header">
          <span class="card-title">{{ title || '电脑变快建议' }}</span>
          <span v-if="!isSuccess" class="status-tag error">写入失败</span>
        </div>

        <div v-if="startupItems.length" class="group-label">开机自启动 · {{ startupItems.length }}</div>
        <div v-for="(item, index) in startupItems" :key="`startup-${index}`" class="check-row">
          <span class="check-name">{{ item.name }}</span>
          <span class="check-purpose">{{ item.reason }}</span>
        </div>

        <div v-if="processes.length" class="group-label">现在先关掉 · {{ processes.length }}</div>
        <div v-for="(item, index) in processes" :key="`process-${index}`" class="check-row">
          <span class="check-name">{{ item.name }}</span>
          <span class="check-purpose">{{ item.reason }}</span>
        </div>

        <div v-if="!startupItems.length && !processes.length" class="empty-hint">未能解析清单内容</div>

        <div v-if="startupItems.length || processes.length" class="card-footer">
          <span class="footer-text">建议已保存到「任务」</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'

interface ProcessItem {
  name: string
  path: string
  reason: string
}

interface StartupItem {
  name: string
  location: string
  command: string
  reason: string
}

const props = defineProps<{
  message: AgentMessage
}>()

const displayContent = computed(() => (props.message.content ?? '').trim())
const isSuccess = computed(() => displayContent.value.startsWith('工具调用结果'))

const parsedArgs = computed<{
  title: string
  processes: ProcessItem[]
  startup_items: StartupItem[]
} | null>(() => {
  const matched = displayContent.value.match(/参数:\s*(\{[\s\S]*?\})\n输出:/)
  if (!matched) return null

  try {
    const parsed = JSON.parse(matched[1])
    return {
      title: typeof parsed.title === 'string' ? parsed.title : '',
      processes: Array.isArray(parsed.processes) ? parsed.processes : [],
      startup_items: Array.isArray(parsed.startup_items) ? parsed.startup_items : [],
    }
  }
  catch {
    return null
  }
})

const title = computed(() => parsedArgs.value?.title ?? '')
const processes = computed(() => parsedArgs.value?.processes ?? [])
const startupItems = computed(() => parsedArgs.value?.startup_items ?? [])
</script>

<style scoped>
.write-checklist-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
}

.checklist-card {
  max-width: min(42rem, 100%);
  display: flex;
  border-radius: 0.75rem;
  overflow: hidden;
  background: #ffffff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06), 0 0 0 1px rgba(0, 0, 0, 0.04);
}

.checklist-card.is-error .accent-strip {
  background: linear-gradient(180deg, #ef4444 0%, #dc2626 100%);
}

.accent-strip {
  width: 0.25rem;
  flex-shrink: 0;
  background: linear-gradient(180deg, #d97706 0%, #b45309 100%);
}

.card-body {
  flex: 1;
  min-width: 0;
  padding: 0.75rem 0.875rem;
  max-height: 20rem;
  overflow: auto;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.card-title {
  font-size: 0.8125rem;
  font-weight: 700;
  color: #0f172a;
}

.status-tag {
  font-size: 0.6875rem;
  line-height: 1;
  padding: 0.2rem 0.45rem;
  border-radius: 0.25rem;
  font-weight: 600;
}

.status-tag.error {
  background: #fef2f2;
  color: #b91c1c;
}

.group-label {
  margin-top: 0.4rem;
  font-size: 0.625rem;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: #94a3b8;
}

.check-row {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
  padding: 0.35rem 0.125rem;
}

.check-name {
  font-size: 0.8125rem;
  font-weight: 600;
  color: #1e293b;
}

.check-purpose {
  font-size: 0.75rem;
  color: #64748b;
}

.card-footer {
  margin-top: 0.5rem;
  padding-top: 0.5rem;
  border-top: 1px dashed #e2e8f0;
}

.footer-text {
  font-size: 0.6875rem;
  color: #94a3b8;
}

.empty-hint {
  font-size: 0.8125rem;
  color: #94a3b8;
  padding: 0.5rem 0;
}
</style>

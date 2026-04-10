<template>
  <div class="write-checklist-message">
    <div class="checklist-card" :class="{ 'is-error': !isSuccess }">
      <div class="accent-strip" />

      <div class="card-body">
        <div class="card-header">
          <span class="card-title">{{ title || '清理清单' }}</span>
          <span v-if="!isSuccess" class="status-tag error">写入失败</span>
        </div>

        <div v-if="items.length" class="check-list">
          <div v-for="(item, index) in items" :key="`${item.path}-${index}`" class="check-row">
            <span class="check-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
                <circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.2" />
                <path d="M5 8.2l2 2 4-4" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
            </span>
            <div class="check-content">
              <span class="check-path" :title="item.path">{{ item.path }}</span>
              <span class="check-purpose">{{ item.purpose }}</span>
            </div>
          </div>
        </div>

        <div v-else class="empty-hint">未能解析清单内容</div>

        <div v-if="items.length" class="card-footer">
          <span class="footer-text">共 {{ items.length }} 项已归档到 Storage Box</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'

interface ChecklistItem {
  path: string
  purpose: string
}

const props = defineProps<{
  message: AgentMessage
}>()

const displayContent = computed(() => (props.message.content ?? '').trim())

const isSuccess = computed(() => displayContent.value.startsWith('工具调用结果'))

const parsedArgs = computed<{ title: string; content: ChecklistItem[] } | null>(() => {
  const matched = displayContent.value.match(/参数:\s*(\{[\s\S]*?\})\n输出:/)
  if (!matched) return null

  try {
    const parsed = JSON.parse(matched[1])
    if (typeof parsed.title === 'string' && Array.isArray(parsed.content)) {
      return parsed as { title: string; content: ChecklistItem[] }
    }
    return null
  }
  catch {
    return null
  }
})

const title = computed(() => parsedArgs.value?.title ?? '')

const items = computed<ChecklistItem[]>(() => parsedArgs.value?.content ?? [])
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
  background: linear-gradient(180deg, #22c55e 0%, #16a34a 100%);
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
  margin-bottom: 0.625rem;
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

.check-list {
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
}

.check-row {
  display: flex;
  align-items: flex-start;
  gap: 0.5rem;
  padding: 0.4rem 0.375rem;
  border-radius: 0.5rem;
  transition: background 0.15s;
}

.check-row:hover {
  background: #f8fafc;
}

.check-icon {
  flex-shrink: 0;
  margin-top: 0.1rem;
  color: #22c55e;
  display: flex;
  align-items: center;
}

.is-error .check-icon {
  color: #94a3b8;
}

.check-content {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.check-path {
  font-size: 0.8125rem;
  font-weight: 500;
  color: #1e293b;
  word-break: break-all;
  line-height: 1.4;
}

.check-purpose {
  font-size: 0.75rem;
  color: #64748b;
  line-height: 1.45;
}

.card-footer {
  margin-top: 0.625rem;
  padding-top: 0.5rem;
  border-top: 1px dashed #e2e8f0;
}

.footer-text {
  font-size: 0.6875rem;
  color: #94a3b8;
  letter-spacing: 0.01em;
}

.empty-hint {
  font-size: 0.8125rem;
  color: #94a3b8;
  padding: 0.5rem 0;
}
</style>

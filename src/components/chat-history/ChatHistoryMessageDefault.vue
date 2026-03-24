<template>
  <div
    v-if="displayContent"
    class="message-item"
    :class="[`message-item--${message.role}`]"
  >
    <span class="message-role">{{ roleLabelMap[message.role] }}</span>
    <p class="message-content">{{ displayContent }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../composables/useAgentHistory'

const props = defineProps<{
  message: AgentMessage
}>()

const roleLabelMap = {
  system: '系统',
  user: '我',
  assistant: '助手',
  tool: '工具',
} as const

const displayContent = computed(() => {
  if (props.message.content && props.message.content.trim()) {
    return props.message.content
  }

  if (props.message.tool_calls?.length) {
    return JSON.stringify(props.message.tool_calls, null, 2)
  }

  if (props.message.tool_call_id) {
    return `tool_call_id: ${props.message.tool_call_id}`
  }

  return ''
})
</script>

<style scoped>
.message-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  max-width: min(42rem, 100%);
  padding: 0.75rem 0.875rem;
  border-radius: 0.875rem;
  background-color: #ffffff;
  border: 0.0625rem solid #e5e7eb;
}

.message-item--user {
  align-self: flex-end;
  background-color: #eef4ff;
  border-color: #c7d2fe;
}

.message-item--assistant {
  align-self: flex-start;
}

.message-item--system {
  align-self: center;
  background-color: #f8fafc;
  border-style: dashed;
}

.message-item--tool {
  align-self: flex-start;
  background-color: #fffbeb;
  border-color: #fcd34d;
}

.message-role {
  font-size: 0.75rem;
  color: #64748b;
}

.message-item--tool .message-role {
  color: #92400e;
}

.message-content {
  margin: 0;
  color: #0f172a;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.6;
}
</style>

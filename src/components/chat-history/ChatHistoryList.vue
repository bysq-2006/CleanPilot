<!-- 这整个vue是一个长页面，没滚动条的，要在外面加 -->
<template>
  <div class="history-panel">
    <div class="message-list">
      <component
        :is="resolveRenderer(message)"
        v-for="(message, index) in visibleMessages"
        :key="`${message.role}-${index}`"
        :message="message"
      />

      <div
        v-if="visibleMessages.length === 0"
        class="empty-placeholder"
      >
        暂无聊天记录
      </div>
    </div>

    <div
      v-if="syncError"
      class="sync-error"
    >
      同步历史记录失败：{{ syncError }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../composables/useAgentHistory'
import ChatHistoryMessageAssistant from './ChatHistoryMessageAssistant.vue'
import ChatHistoryMessageUnknown from './ChatHistoryMessageUnknown.vue'
import ChatHistoryMessageUser from './ChatHistoryMessageUser.vue'

const props = defineProps<{
  messages: AgentMessage[]
  syncError: string | null
}>()

const visibleMessages = computed(() => props.messages.filter((message) => message.role !== 'system'))

const resolveRenderer = (message: AgentMessage) => {
  if (message.role === 'assistant') {
    return ChatHistoryMessageAssistant
  }

  if (message.role === 'user') {
    return ChatHistoryMessageUser
  }

  return ChatHistoryMessageUnknown
}
</script>

<style scoped>
.history-panel {
  padding: 1rem 0.75rem;
}

.message-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.empty-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 8rem;
  color: #94a3b8;
  font-size: 0.875rem;
}

.sync-error {
  margin-top: 0.75rem;
  color: #dc2626;
  font-size: 0.75rem;
}
</style>

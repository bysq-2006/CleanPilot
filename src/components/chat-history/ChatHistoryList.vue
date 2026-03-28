<!-- 这整个vue是一个长页面，没滚动条的，要在外面加 -->
<template>
  <div class="history-panel" @click="debugPrintHistory">
    <div class="message-list">
      <template v-for="(message, index) in messages" :key="`${message.role}-${index}`">
        <ChatHistoryMessageAssistant v-if="message.role === 'assistant'" :message="message" />

        <ChatHistoryMessageUser v-else-if="message.role === 'user'" :message="message" />

        <ChatHistoryMessageUnknown v-else :message="message" />
      </template>

      <div v-if="messages.length === 0" class="empty-placeholder">
        暂无聊天记录
      </div>
    </div>

    <div v-if="pendingToolCall" class="pending-tool-call">
      <pre>{{ pendingToolCall }}</pre>
    </div>

    <div v-if="syncError" class="sync-error">
      同步历史记录失败：{{ syncError }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../composables/useAgentHistory'
import ChatHistoryMessageAssistant from './messages/ChatHistoryMessageAssistant.vue'
import ChatHistoryMessageUnknown from './ChatHistoryMessageUnknown.vue'
import ChatHistoryMessageUser from './messages/ChatHistoryMessageUser.vue'

const props = defineProps<{
  messages: AgentMessage[]
  syncError: string | null
}>()

const debugPrintHistory = () => {
  console.log('chat history messages:', props.messages)
}

// 从后往前找，找到第一个 assistant 消息里未完成的 tool call
const pendingToolCall = computed<any>(() => {
  const completedToolCallIds = new Set<string>()

  for (let index = props.messages.length - 1; index >= 0; index -= 1) {
    const message = props.messages[index]

    if (message.role === 'tool' && message.tool_call_id) {
      completedToolCallIds.add(message.tool_call_id)
      continue
    }

    if (message.role !== 'assistant' || !message.tool_calls?.length) {
      continue
    }

    for (const toolCall of message.tool_calls) {
      console.log('checking tool call', toolCall, completedToolCallIds)
      if (!completedToolCallIds.has(toolCall.id)) {
        console.log('pending tool call found', toolCall)
        return {
          ...toolCall,
          ready: true,
        }
      }
    }
  }

  return null
})
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

.pending-tool-call {
  margin-top: 0.75rem;
}

.sync-error {
  margin-top: 0.75rem;
  color: #dc2626;
  font-size: 0.75rem;
}
</style>

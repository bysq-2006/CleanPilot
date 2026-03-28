<template>
  <div
    v-if="displayContent"
    class="tool-message"
  >
    <div class="tool-title">工具调用（{{ displayToolName }}）</div>
    <pre class="tool-content">{{ displayContent }}</pre>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../composables/useAgentHistory'

const props = defineProps<{
  message: AgentMessage
}>()

const displayToolName = computed(() => (props.message.tool_name ?? 'unknown_tool').trim())
const displayContent = computed(() => (props.message.content ?? '').trim())
</script>

<style scoped>
.tool-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.5rem;
}

.tool-title {
  color: #64748b;
  font-size: 0.8125rem;
  font-weight: 600;
}

.tool-content {
  margin: 0;
  max-width: min(42rem, 100%);
  padding: 0.8125rem 0.9375rem;
  border-radius: 0.875rem;
  border: 0.0625rem solid #dbe4ee;
  background: #ffffff;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.9375rem;
}
</style>

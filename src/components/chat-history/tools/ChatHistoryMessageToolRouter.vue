<template>
  <component :is="selectedComponent" :message="message" />
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../composables/useAgentHistory'
import ChatHistoryMessageToolGeneric from './ChatHistoryMessageToolGeneric.vue'

const props = defineProps<{
  message: AgentMessage
}>()

const toolComponentMap: Record<string, unknown> = {
  list_directory: ChatHistoryMessageToolGeneric,
  http_request: ChatHistoryMessageToolGeneric,
}

const selectedComponent = computed(() => {
  const toolName = (props.message.tool_name ?? '').trim()
  return toolComponentMap[toolName] ?? ChatHistoryMessageToolGeneric
})
</script>

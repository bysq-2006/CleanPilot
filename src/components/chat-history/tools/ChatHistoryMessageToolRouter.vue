<template>
  <component :is="selectedComponent" :message="message" />
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../composables/useAgentHistory'
import ToolMessageFallback from './ToolMessageFallback.vue'
import ToolDiskInfoMessage from './disk_cleanup/ToolDiskInfoMessage.vue'
import ToolFindLargeEntriesMessage from './disk_cleanup/ToolFindLargeEntriesMessage.vue'
import ToolListDirectoryMessage from './disk_cleanup/ToolListDirectoryMessage.vue'
import ToolFileReadMessage from './utility/ToolFileReadMessage.vue'
import ToolHttpRequestMessage from './utility/ToolHttpRequestMessage.vue'

const props = defineProps<{
  message: AgentMessage
}>()

// 根据工具名称选择对应的组件
const toolComponentMap: Record<string, unknown> = {
  http_request: ToolHttpRequestMessage,
  file_read: ToolFileReadMessage,
  find_large_entries: ToolFindLargeEntriesMessage,
  get_disk_info: ToolDiskInfoMessage,
  list_directory: ToolListDirectoryMessage,
}

const selectedComponent = computed(() => {
  const toolName = (props.message.tool_name ?? '').trim()
  return toolComponentMap[toolName] ?? ToolMessageFallback
})
</script>

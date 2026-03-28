<template>
  <div class="tool-fallback-message">
    <CommonFrameBox max-height="16rem">
      <pre class="tool-fallback-content">{{ displayContent }}</pre>
    </CommonFrameBox>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../composables/useAgentHistory'
import CommonFrameBox from '../../CommonFrameBox.vue'

const props = defineProps<{
  message: AgentMessage
}>()

const displayContent = computed(() => (props.message.content ?? '').trim())
</script>

<style scoped>
.tool-fallback-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
}

.tool-fallback-content {
  margin: 0;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.875rem;
}
</style>

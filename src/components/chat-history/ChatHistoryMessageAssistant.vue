<template>
  <div
    v-if="displayContent"
    class="assistant-message"
  >
    <img
      class="assistant-avatar"
      src="/robot.svg"
      alt="助手"
    >

    <div class="assistant-card">
      <div class="assistant-name">Memora</div>
      <div
        class="assistant-content markdown-body"
        v-html="renderedContent"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MarkdownIt from 'markdown-it'

import type { AgentMessage } from '../../composables/useAgentHistory'

const markdown = new MarkdownIt({
  breaks: true,
  linkify: true,
})

const props = defineProps<{
  message: AgentMessage
}>()

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

const renderedContent = computed(() => markdown.render(displayContent.value))
</script>

<style scoped>
.assistant-message {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  max-width: min(46rem, 100%);
}

.assistant-avatar {
  width: 2.25rem;
  height: 2.25rem;
  flex-shrink: 0;
}

.assistant-card {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  min-width: 0;
}

.assistant-name {
  font-size: 0.8125rem;
  line-height: 1.25rem;
  color: #64748b;
  font-weight: 600;
}

.assistant-content {
  color: #334155;
  word-break: break-word;
  line-height: 1.65;
  font-size: 1rem;
}

.assistant-content :deep(p) {
  margin: 0;
}

.assistant-content :deep(p + p) {
  margin-top: 0.875rem;
}

.assistant-content :deep(h1),
.assistant-content :deep(h2),
.assistant-content :deep(h3),
.assistant-content :deep(h4),
.assistant-content :deep(h5),
.assistant-content :deep(h6) {
  margin: 1rem 0 0.5rem;
  color: #0f172a;
  line-height: 1.35;
  font-size: 1.1em;
}

.assistant-content :deep(ul),
.assistant-content :deep(ol) {
  margin: 0.75rem 0;
  padding-left: 1.5rem;
}

.assistant-content :deep(li + li) {
  margin-top: 0.25rem;
}

.assistant-content :deep(code) {
  padding: 0.125rem 0.375rem;
  border-radius: 0.375rem;
  background: #e2e8f0;
  color: #0f172a;
  font-size: 0.9em;
  font-family: Consolas, 'Courier New', monospace;
}

.assistant-content :deep(pre) {
  margin: 0.875rem 0 0;
  padding: 0.875rem 1rem;
  border-radius: 0.875rem;
  background: #0f172a;
  overflow-x: auto;
}

.assistant-content :deep(pre code) {
  padding: 0;
  background: transparent;
  color: #e2e8f0;
}

.assistant-content :deep(a) {
  color: #2563eb;
  text-decoration: none;
}

.assistant-content :deep(a:hover) {
  text-decoration: underline;
}

.assistant-content :deep(blockquote) {
  margin: 0.875rem 0 0;
  padding-left: 0.875rem;
  border-left: 0.1875rem solid #cbd5e1;
  color: #475569;
}
</style>

<template>
  <div
    v-if="displaySegments.length"
    class="unknown-message"
  >
  未知消息类型：
    <div
      v-for="(segment, index) in displaySegments"
      :key="index"
      class="unknown-segment"
    >
      <p class="unknown-content">{{ segment }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  message: any
}>()

const displayContent = computed(() => props.message.content.trim())

const displaySegments = computed(() => {
  return displayContent.value
    .split(/\n{2,}/)
    .map((segment: any) => segment.trim())
    .filter(Boolean)
})
</script>

<style scoped>
.unknown-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.5rem;
}

.unknown-segment {
  max-width: min(42rem, 100%);
  padding: 0.8125rem 0.9375rem;
  border-radius: 0.875rem;
  border: 0.0625rem solid #dbe4ee;
  background: #ffffff;
}

.unknown-content {
  margin: 0;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.9375rem;
}
</style>

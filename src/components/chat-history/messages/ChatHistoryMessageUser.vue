<template>
  <div
    v-if="displayContent"
    class="user-message"
  >
    <div class="user-bubble">
      <p class="user-content">{{ displayContent }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  message: any
}>()

const displayContent = computed(() => props.message.content.trim())
</script>

<style scoped>
.user-message {
  display: flex;
  justify-content: flex-end;
  width: 100%;
}

.user-bubble {
  max-width: min(40rem, 100%);
  padding: 0.8125rem 0.9375rem;
  border-radius: 1.125rem;
  background: #eceff0;
}

.user-content {
  margin: 0;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.9375rem;
}
</style>

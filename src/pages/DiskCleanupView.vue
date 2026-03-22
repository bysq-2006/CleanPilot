<template>
  <div class="history-scroll-container">
    <ChatHistoryList :messages="messages" :sync-error="syncError" />
  </div>

  <div class="composer-dock">
    <ChatComposer />
  </div>
</template>

<script setup lang="ts">
import ChatComposer from '../components/ChatComposer.vue'
import ChatHistoryList from '../components/chat-history/ChatHistoryList.vue'
import { useAgentHistory } from '../composables/useAgentHistory'

const { messages, syncError } = useAgentHistory()
</script>

<style scoped>
.history-scroll-container {
  height: 100%;
  min-height: 0;
  overflow-y: auto;
}

.composer-dock {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 1;
  width: 100%;
}
</style>

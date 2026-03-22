<template>
  <div class="disk-cleanup-view">
    <div class="history-scroll-container">
      <ChatHistoryList :messages="messages" :sync-error="syncError" />
    </div>

    <ChatComposer class="composer-dock" />
  </div>
</template>

<script setup lang="ts">
import ChatComposer from '../components/ChatComposer.vue'
import ChatHistoryList from '../components/chat-history/ChatHistoryList.vue'
import { useAgentHistory } from '../composables/useAgentHistory'

const { messages, syncError } = useAgentHistory()
</script>

<style scoped>
.disk-cleanup-view {
  height: 100%;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.history-scroll-container {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}

.composer-dock {
  width: 100%;
  flex-shrink: 0;
}
</style>

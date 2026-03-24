<template>
  <div class="disk-cleanup-view">
    <div class="history-scroll-container">
      <ChatHistoryList :messages="messages" :sync-error="syncError" />
    </div>

    <ChatComposer class="composer-dock" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import ChatComposer from '../components/ChatComposer.vue'
import ChatHistoryList from '../components/chat-history/ChatHistoryList.vue'
import { agentHistoryStore } from '../composables/useAgentHistory'

const messages = computed(() => agentHistoryStore.history.value)
const syncError = computed(() => agentHistoryStore.syncError.value)
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

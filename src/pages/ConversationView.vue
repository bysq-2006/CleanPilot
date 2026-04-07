<template>
  <div class="disk-cleanup-view">
    <div ref="historyScrollContainer" class="history-scroll-container" @scroll="handleHistoryScroll">
      <ChatHistoryList :messages="messages" :sync-error="syncError" />
      <AgentWorkingIndicator class="AgentWorkingshow"/>
    </div>

    <ChatComposer class="composer-dock" />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue'

import AgentWorkingIndicator from '../components/AgentWorkingIndicator.vue'
import ChatComposer from '../components/ChatComposer.vue'
import ChatHistoryList from '../components/chat-history/ChatHistoryList.vue'
import { AgentHistoryStore } from '../composables/useAgentHistory'

const agentHistoryStore = new AgentHistoryStore()
const isPinnedToBottom = ref(true)

const messages = computed(() => agentHistoryStore.history.value)
const syncError = computed(() => agentHistoryStore.syncError.value)


//只有用户手动滚动时，才去修改是否自动滚动
const historyScrollContainer = ref<HTMLDivElement | null>(null)
const bottomThresholdPx = 12

const checkIsNearBottom = (element: HTMLDivElement) => {
  return element.scrollHeight - element.scrollTop - element.clientHeight <= bottomThresholdPx
}

const scrollToBottom = () => {
  const element = historyScrollContainer.value

  if (!element) return

  element.scrollTop = element.scrollHeight
}

const handleHistoryScroll = () => {
  const element = historyScrollContainer.value

  if (!element) return

  isPinnedToBottom.value = checkIsNearBottom(element)
}

watch(
  () => agentHistoryStore.history.value,
  async () => {
    if (!isPinnedToBottom.value) return

    await nextTick()
    scrollToBottom()
  },
  { deep: true },
)

onMounted(async () => {
  await nextTick()
  scrollToBottom()
  handleHistoryScroll()
})
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

.AgentWorkingshow {
  margin-left: 3.8rem;
}

.composer-dock {
  width: 100%;
  flex-shrink: 0;
}
</style>

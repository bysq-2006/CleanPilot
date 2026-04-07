<template>
  <div v-if="isWorking" class="agent-working-indicator" role="status" aria-live="polite">
    <span class="agent-working-indicator__text">CleanPilot 正在思考并生成回复</span>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'

const pollIntervalMs = 200
const status = ref('idle')

const isWorking = computed(() => status.value === 'chatting')

const syncStatus = async () => {
  try {
    status.value = await invoke<string>('get_agent_status')
  }
  catch {
    status.value = 'idle'
  }
}

const startPolling = async () => {
  await syncStatus()

  window.setInterval(() => {
    void syncStatus()
  }, pollIntervalMs)
}

onMounted(() => {
  void startPolling()
})
</script>

<style scoped>
.agent-working-indicator {
  display: inline-flex;
  align-items: center;
  max-width: min(100%, 32rem);
  overflow: hidden;
}

.agent-working-indicator__text {
  position: relative;
  display: inline-block;
  font-size: 0.8125rem;
  line-height: 1.5;
  font-weight: 400;
  white-space: nowrap;
  color: #8a94a3;
  background-image: linear-gradient(
    110deg,
    rgba(138, 148, 163, 0.92) 0%,
    rgba(138, 148, 163, 0.92) 44%,
    rgba(255, 255, 255, 0.96) 50%,
    rgba(138, 148, 163, 0.92) 56%,
    rgba(138, 148, 163, 0.92) 100%
  );
  background-size: 300% 100%;
  background-position: 160% 50%;
  background-repeat: no-repeat;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: text-shimmer 2.8s linear infinite;
}

@supports not ((-webkit-background-clip: text) or (background-clip: text)) {
  .agent-working-indicator__text {
    color: #8a94a3;
    background-image: none;
    -webkit-text-fill-color: initial;
  }
}

@keyframes text-shimmer {
  0% {
    background-position: 160% 50%;
  }

  100% {
    background-position: -160% 50%;
  }
}

@media (prefers-reduced-motion: reduce) {
  .agent-working-indicator__text {
    animation: none;
    background-position: 50% 50%;
  }
}
</style>

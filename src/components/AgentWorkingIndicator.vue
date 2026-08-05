<!-- 显示 Agent 当前是否正在处理任务。 -->
<template>
  <div v-if="isWorking" class="agent-working-indicator" role="status" aria-live="polite">
    <span
      class="agent-working-indicator__text"
      data-text="CleanPilot 正在思考并生成回复"
    >CleanPilot 正在思考并生成回复</span>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'

import { startAgentStatusPolling, useAgentStatus } from '../composables/useAgentStatus'

const { isWorking } = useAgentStatus()

onMounted(() => {
  startAgentStatusPolling()
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
  color: rgba(138, 148, 163, 0.92);
  white-space: nowrap;
}

.agent-working-indicator__text::after {
  content: attr(data-text);
  position: absolute;
  inset: 0;
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
  pointer-events: none;
  animation: text-shimmer 2.8s linear infinite;
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

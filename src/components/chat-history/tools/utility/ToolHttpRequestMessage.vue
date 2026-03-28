<template>
  <div class="http-tool-message">
    <CommonFrameBox max-height="15rem">
      <div class="http-tool-visual">
        <div class="device-icon" aria-label="客户端设备">
          <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" role="img">
            <path
              d="M554.666667 682.666667v42.666666h85.333333v85.333334H384v-85.333334h85.333333v-42.666666H170.666667V213.333333h682.666666v469.333334h-298.666666z m0-85.333334h213.333333V298.666667H256v298.666666h298.666667z"
              fill="#444444" />
          </svg>
        </div>

        <div class="transfer-track" :class="{ preparing: isPreparing, 'with-status': !isPreparing && !!statusCode }">
          <span v-if="!isPreparing && statusCode" class="status-code">{{ statusCode }}</span>
        </div>

        <div class="server-icon" aria-label="服务端资源">
          <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" role="img">
            <path d="M0 0" fill="" />
            <path
              d="M128 608h768v192H128v-192z m0-256h768v192H128v-192z m32-140.8h704l28.8 73.6H131.2L160 211.2zM131.2 160L64 284.8V864h896V284.8L896 160H131.2z"
              fill="#444444" />
            <path d="M768 416h64v64h-64zM768 672h64v64h-64z" fill="#444444" />
          </svg>
        </div>
      </div>
    </CommonFrameBox>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { AgentMessage } from '../../../../composables/useAgentHistory'
import CommonFrameBox from '../../../CommonFrameBox.vue'

const props = defineProps<{
  message: AgentMessage
}>()

const isPreparing = computed(() => props.message.ready === true)

const statusCode = computed(() => {
  const content = props.message.content ?? ''
  const matched = content.match(/状态码:\s*([A-Za-z0-9./_-]+)/)
  return matched?.[1] ?? null
})
</script>

<style scoped>
.http-tool-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
}

.http-tool-visual {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: min(42rem, 100%);
}

.device-icon,
.server-icon {
  width: 2rem;
  height: 2rem;
  flex-shrink: 0;
}

.device-icon svg,
.server-icon svg {
  width: 100%;
  height: 100%;
  display: block;
}

.transfer-track {
  position: relative;
  flex: 1;
  min-width: 5rem;
  height: 0.125rem;
  background: #111111;
}

.transfer-track.with-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  height: 1.25rem;
  background: transparent;
}

.transfer-track.with-status::before,
.transfer-track.with-status::after {
  content: '';
  flex: 1;
  height: 0.125rem;
  background: #111111;
}

.status-code {
  flex-shrink: 0;
  padding: 0.1rem 0.45rem;
  border-radius: 999px;
  border: 0.0625rem solid #111111;
  background: #ffffff;
  color: #111111;
  font-size: 0.75rem;
  line-height: 1;
  font-weight: 600;
}

.transfer-track.preparing {
  height: 0.2rem;
  background: repeating-linear-gradient(
    90deg,
    #111111 0,
    #111111 0.35rem,
    transparent 0.35rem,
    transparent 0.62rem
  );
  -webkit-mask-image: linear-gradient(
    90deg,
    transparent 0%,
    #000000 43%,
    #000000 50%,
    transparent 84%
  );
  -webkit-mask-size: 250% 100%;
  -webkit-mask-position: 0% 0;
  mask-image: linear-gradient(
    90deg,
    transparent 0%,
    #000000 43%,
    #000000 50%,
    transparent 84%
  );
  mask-size: 250% 100%;
  mask-position: 0% 0;
  animation: mask-swing 1.2s ease-in-out infinite alternate;
}

@keyframes mask-swing {
  0% {
    -webkit-mask-position: 0% 0;
    mask-position: 0% 0;
  }

  100% {
    -webkit-mask-position: 100% 0;
    mask-position: 100% 0;
  }
}
</style>

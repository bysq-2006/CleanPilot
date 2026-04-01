<template>
  <div class="file-read-message">
    <CommonFrameBox max-height="18rem">
      <template v-if="parsed.valid">
        <div class="file-read-header">
          <div class="file-icon" aria-label="文件内容读取结果">
            <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" role="img">
              <path
                d="M256 96h352l160 160v672H256V96zm384 45.248V288h146.752L640 141.248zM341.312 416h341.376v64H341.312v-64zm0 160h341.376v64H341.312v-64zm0 160h213.376v64H341.312v-64z"
                fill="#444444" />
            </svg>
          </div>

          <div class="file-meta">
            <div class="file-path" :title="parsed.path">{{ parsed.path }}</div>
            <div class="file-stats">
              <span>总字符 {{ parsed.totalChars }}</span>
              <span>返回 {{ parsed.returnedChars }}</span>
              <span :class="['truncate-badge', parsed.truncated ? 'warning' : 'safe']">
                {{ parsed.truncated ? '已截断' : '完整返回' }}
              </span>
            </div>
          </div>
        </div>

        <pre class="file-content">{{ parsed.content }}</pre>
      </template>

      <pre v-else class="raw-content">{{ displayContent }}</pre>
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

const displayContent = computed(() => (props.message.content ?? '').trim())

const parsed = computed(() => {
  const content = displayContent.value

  const path = content.match(/文件路径:\s*(.+)/)?.[1]?.trim() ?? ''
  const totalChars = Number(content.match(/字符数:\s*(\d+)/)?.[1] ?? NaN)
  const returnedChars = Number(content.match(/返回字符数:\s*(\d+)/)?.[1] ?? NaN)
  const truncatedText = content.match(/是否截断:\s*(.+)/)?.[1]?.trim() ?? ''
  const fileContent = content.match(/文件内容:\n([\s\S]*)$/)?.[1] ?? ''

  const valid = !!path
    && Number.isFinite(totalChars)
    && Number.isFinite(returnedChars)

  return {
    valid,
    path,
    totalChars,
    returnedChars,
    truncated: truncatedText === '是',
    content: fileContent,
  }
})
</script>

<style scoped>
.file-read-message {
  margin: 0.8rem;
  margin-left: 2.9rem;
}

.file-read-header {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.file-icon {
  width: 2rem;
  height: 2rem;
  flex-shrink: 0;
}

.file-icon svg {
  width: 100%;
  height: 100%;
  display: block;
}

.file-meta {
  min-width: 0;
  flex: 1;
}

.file-path {
  font-size: 0.8125rem;
  font-weight: 700;
  color: #0f172a;
  word-break: break-all;
  line-height: 1.5;
}

.file-stats {
  margin-top: 0.35rem;
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem 0.6rem;
  font-size: 0.75rem;
  color: #64748b;
}

.truncate-badge {
  padding: 0.08rem 0.45rem;
  border-radius: 999px;
  border: 0.0625rem solid currentColor;
  font-weight: 600;
}

.truncate-badge.safe {
  color: #4db071;
}

.truncate-badge.warning {
  color: #d97706;
}

.file-content,
.raw-content {
  margin: 0;
  padding: 0.75rem;
  border-radius: 0.75rem;
  background: #f8fafc;
  border: 0.0625rem solid #e2e8f0;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.65;
  font-size: 0.8125rem;
  overflow: auto;
}
</style>

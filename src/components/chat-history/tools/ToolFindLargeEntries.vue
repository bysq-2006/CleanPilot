<template>
  <div class="tool-card">
    <div class="tool-card-header">
      <div>
        <div class="tool-title">大文件扫描</div>
        <div class="tool-subtitle">递归扫描目录中的大文件和大文件夹</div>
      </div>

      <span
        class="tool-status"
        :class="`tool-status--${tool.status}`"
      >
        {{ statusText }}
      </span>
    </div>

    <div class="tool-section">
      <div class="tool-label">扫描参数</div>
      <div class="tool-kv-list">
        <div class="tool-kv-item">
          <span class="tool-kv-key">目录</span>
          <span class="tool-kv-value">{{ parsedArguments.path || '—' }}</span>
        </div>
        <div class="tool-kv-item">
          <span class="tool-kv-key">阈值</span>
          <span class="tool-kv-value">{{ minSizeText }}</span>
        </div>
      </div>
    </div>

    <div
      v-if="tool.status === 'ready'"
      class="tool-progress"
    >
      正在扫描中，请稍候…
    </div>

    <div
      v-if="tool.status === 'done' && resultLines.length"
      class="tool-section"
    >
      <div class="tool-label">扫描结果</div>
      <ul class="tool-result-list">
        <li
          v-for="(line, index) in resultLines"
          :key="index"
          class="tool-result-item"
        >
          {{ line }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import type { RenderableToolItem } from '../../../composables/useRenderableHistory'

const props = defineProps<{
  tool: RenderableToolItem
}>()

const parsedArguments = computed(() => {
  try {
    return JSON.parse(props.tool.arguments) as {
      path?: string
      min_size_mb?: number
    }
  }
  catch {
    return {}
  }
})

const minSizeText = computed(() => {
  const value = parsedArguments.value.min_size_mb
  return typeof value === 'number' ? `${value} MB` : '—'
})

const statusText = computed(() => props.tool.status === 'done' ? '已完成' : '准备中')

const resultLines = computed(() => {
  return props.tool.result
    .split('\n')
    .map((line) => line.trim())
    .filter(Boolean)
    .slice(0, 12)
})
</script>

<style scoped>
.tool-card {
  width: min(42rem, 100%);
  padding: 0.875rem 0.9375rem;
  border-radius: 1rem;
  border: 0.0625rem solid #dbe4ee;
  background: #ffffff;
  box-shadow: 0 0.25rem 0.75rem rgb(15 23 42 / 0.04);
}

.tool-card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
}

.tool-title {
  color: #0f172a;
  font-size: 0.9375rem;
  font-weight: 600;
  line-height: 1.4;
}

.tool-subtitle {
  margin-top: 0.1875rem;
  color: #64748b;
  font-size: 0.75rem;
  line-height: 1.4;
}

.tool-status {
  flex-shrink: 0;
  padding: 0.25rem 0.625rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  line-height: 1.2;
  font-weight: 600;
}

.tool-status--ready {
  background: #eff6ff;
  color: #2563eb;
}

.tool-status--done {
  background: #ecfdf5;
  color: #059669;
}

.tool-section {
  margin-top: 0.75rem;
}

.tool-label {
  margin-bottom: 0.375rem;
  color: #64748b;
  font-size: 0.75rem;
  font-weight: 600;
}

.tool-kv-list {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.tool-kv-item {
  display: flex;
  gap: 0.75rem;
  font-size: 0.8125rem;
  line-height: 1.5;
}

.tool-kv-key {
  width: 3rem;
  flex-shrink: 0;
  color: #94a3b8;
}

.tool-kv-value {
  color: #334155;
  word-break: break-all;
}

.tool-progress {
  margin-top: 0.75rem;
  color: #2563eb;
  font-size: 0.8125rem;
  line-height: 1.5;
}

.tool-result-list {
  margin: 0;
  padding-left: 1rem;
  color: #334155;
  font-size: 0.8125rem;
  line-height: 1.6;
}

.tool-result-item + .tool-result-item {
  margin-top: 0.25rem;
}
</style>

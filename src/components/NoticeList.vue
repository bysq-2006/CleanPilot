<template>
  <div v-if="notices.length" class="notice-list-wrap">
    <transition-group 
      name="notice-fade" 
      tag="div" 
      class="notice-list"
      appear
    >
      <div
        v-for="item in notices"
        :key="item.id"
        class="notice-item"
        :class="`notice-item--${item.type}`"
      >
        <span class="notice-text">{{ item.text }}</span>
      </div>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { useNoticeCenter } from '../composables/useNoticeCenter'

const { notices } = useNoticeCenter()
</script>

<style scoped>
.notice-list-wrap {
  position: absolute;
  left: 50%;
  top: 2rem;
  transform: translateX(-50%);
  width: min(36rem, calc(100% - 2rem));
  pointer-events: none;
  z-index: 60;
}

.notice-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  position: relative; /* 新增：给transition-group提供定位上下文 */
}

.notice-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.875rem 1rem;
  border-radius: 0.875rem;
  border: 0.0625rem solid #d5e0de;
  box-shadow: 0 0.375rem 0.875rem rgba(25, 40, 36, 0.08);
  backdrop-filter: blur(0.5rem);
  background: rgba(248, 251, 250, 0.96);
  position: relative; /* 新增：让transition-group能捕获元素状态 */
  width: 100%; /* 新增：固定宽度，避免动画时布局抖动 */
}

/* 以下样式保留你的原有代码 */
.notice-item--success {
  border-color: #b9dfc2;
  background: #eef8f1;
}

.notice-item--warning {
  border-color: #edd89a;
  background: #fff6e4;
}

.notice-item--error {
  border-color: #ebb7b1;
  background: #fef0ef;
}

.notice-text {
  color: #31403e;
  font-size: 0.875rem;
  font-weight: 500;
  line-height: 1.4;
  word-break: break-word;
}

.notice-fade-enter-active,
.notice-fade-leave-active {
  transition: opacity 0.5s ease;
}

.notice-fade-enter-from,
.notice-fade-leave-to {
  opacity: 0;
}

.notice-fade-enter-to,
.notice-fade-leave-from {
  opacity: 1;
}
</style>
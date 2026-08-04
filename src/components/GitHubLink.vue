<!-- 以休息小猫动效链接到本项目的 GitHub 仓库。 -->
<template>
  <button class="github-link" type="button" title="在 GitHub 查看项目" aria-label="在 GitHub 查看项目" @click="openRepository">
    <svg class="github-link-icon cat-icon" viewBox="0 0 24 24" aria-hidden="true">
      <path d="M5 17.5v-2.2c0-3.8 2.7-6.3 7-6.3s7 2.5 7 6.3v2.2M7.2 10 6.4 6.7 9.5 8.3M16.8 10l.8-3.3-3.1 1.6M8.5 13c.7.6 1.5.6 2.2 0M13.3 13c.7.6 1.5.6 2.2 0M5.1 15.2c-2.5-.1-2.8 2.6-.7 3.2 1.1.3 2.1-.2 2.6-.9" />
    </svg>

    <svg class="github-link-icon github-icon" viewBox="0 0 24 24" aria-hidden="true">
      <path d="M12 .7a11.5 11.5 0 0 0-3.6 22.4c.6.1.8-.3.8-.6v-2.2c-3.3.7-4-1.4-4-1.4-.5-1.4-1.3-1.8-1.3-1.8-1.1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1.1 1.8 2.8 1.3 3.5 1 .1-.8.4-1.3.8-1.6-2.7-.3-5.5-1.3-5.5-5.7 0-1.3.5-2.3 1.2-3.1-.1-.3-.5-1.6.1-3.1 0 0 1-.3 3.2 1.2a11 11 0 0 1 5.8 0C17.1 4.8 18.1 5 18.1 5c.6 1.5.2 2.8.1 3.1.8.8 1.2 1.8 1.2 3.1 0 4.4-2.8 5.4-5.5 5.7.4.4.8 1.1.8 2.1v3.4c0 .3.2.7.8.6A11.5 11.5 0 0 0 12 .7Z" />
    </svg>
  </button>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

const repositoryUrl = 'https://github.com/bysq-2006/CleanPilot'
const openRepository = () => void openUrl(repositoryUrl)
</script>

<style scoped>
.github-link {
  position: relative;
  width: 2.6rem;
  height: 2.6rem;
  border: 0;
  border-radius: 0.8rem;
  background: transparent;
  color: #84938f;
  cursor: pointer;
  transition: background-color 0.25s ease, color 0.25s ease;
}

.github-link-icon {
  position: absolute;
  inset: 50% auto auto 50%;
  width: 1.55rem;
  height: 1.55rem;
  transition: opacity 0.28s ease, transform 0.28s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.cat-icon {
  fill: none;
  stroke: currentColor;
  stroke-width: 1.5;
  stroke-linecap: round;
  stroke-linejoin: round;
  transform: translate(-50%, -46%) scale(1);
}

.github-icon {
  fill: currentColor;
  opacity: 0;
  transform: translate(-50%, -35%) scale(0.72) rotate(-6deg);
}

.github-link:is(:hover, :focus-visible) {
  outline: none;
  background: rgba(46, 110, 92, 0.09);
  color: #2e6e5c;
}

.github-link:is(:hover, :focus-visible) .cat-icon {
  opacity: 0;
  transform: translate(-50%, -62%) scale(0.72);
}

.github-link:is(:hover, :focus-visible) .github-icon {
  opacity: 1;
  transform: translate(-50%, -50%) scale(1) rotate(0);
}

@media (prefers-reduced-motion: reduce) {
  .github-link,
  .github-link-icon {
    transition: none;
  }
}
</style>

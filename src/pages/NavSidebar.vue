<template>
  <div class="nav-sidebar">
    <div class="nav-top-placeholder">
      <span class="placeholder-dot">•</span>
    </div>

    <button
      v-for="item in navItems"
      :key="item.key"
      class="nav-item"
      :class="{ 'is-active': route.path === item.to }"
      type="button"
      @click="handleNavClick(item.to)"
    >
      <span class="nav-icon-pill">
        <span
          class="nav-icon nav-icon-mask"
          :style="{ '--icon-url': `url(${item.icon})` }"
          aria-hidden="true"
        />
      </span>
      <span class="nav-label">{{ item.label }}</span>
    </button>

    <div class="nav-footer-version">v{{ appVersion }}</div>
  </div>
</template>

<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app'
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()

const navItems = [
  { key: 'disk-cleanup', label: '磁盘清理', icon: '/DiskCleanup.svg', to: '/' },
  { key: 'settings', label: '设置', icon: '/Settings.svg', to: '/settings' },
]

function handleNavClick(path: string) {
  router.push(path)
}

// 显示应用版本号
const appVersion = ref('0.0.0')
onMounted(async () => {
  appVersion.value = await getVersion()
})
</script>

<style scoped>
.nav-sidebar {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.nav-top-placeholder {
  height: 3.125rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder-dot {
  font-size: 1.25rem;
  line-height: 1;
  color: #9da7a6;
}

.nav-item {
  width: 100%;
  max-width: 4.75rem;
  margin: 0 auto;
  border: 0;
  background: transparent;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
  padding: 0.75rem 0.25rem;
  color: #7f8a89;
  cursor: pointer;
  transition: color 0.25s ease;
}

.nav-icon-pill {
  width: 2.6rem;
  height: 1.75rem;
  border-radius: 999px;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.25s ease;
}

.nav-icon-pill::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: linear-gradient(135deg, #f3fbf6 0%, #dff3e7 100%);
  opacity: 0;
  transition: opacity 0.25s ease;
}

.nav-icon {
  width: 1.125rem;
  height: 1.125rem;
  position: relative;
  z-index: 1;
}

.nav-icon-mask {
  background-color: currentColor;
  -webkit-mask-image: var(--icon-url);
  mask-image: var(--icon-url);
  -webkit-mask-repeat: no-repeat;
  mask-repeat: no-repeat;
  -webkit-mask-position: center;
  mask-position: center;
  -webkit-mask-size: contain;
  mask-size: contain;
}

.nav-label {
  font-size: 0.6875rem;
  line-height: 1.2;
  color: currentColor;
  text-align: center;
}

.nav-item.is-active {
  color: #3ea66b;
}

.nav-item.is-active .nav-icon-pill {
  transform: translateY(-1px);
}

.nav-item.is-active .nav-icon-pill::before {
  opacity: 1;
}

.nav-footer-version {
  margin-top: auto;
  padding: 0.75rem 0.5rem 1rem;
  text-align: center;
  font-size: 0.7rem;
  line-height: 1;
  color: #9ca7a5;
  user-select: none;
}
</style>


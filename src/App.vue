<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import NoticeList from "./components/NoticeList.vue";
import NavSidebar from "./pages/NavSidebar.vue";

const appWindow = getCurrentWindow();

async function handleStartDragging() {
  await appWindow.startDragging();
}

async function handleClose() {
  await appWindow.close();
}
</script>

<template>
  <div class="app-shell">
    <aside class="left-nav">
      <NavSidebar />
    </aside>

    <section class="right-panel">
      <header class="window-titlebar" @mousedown.left="handleStartDragging">
        <div class="window-actions">
          <button class="action-btn close-btn" @mousedown.stop @click="handleClose">✕</button>
        </div>
      </header>

      <div class="right-content">
        <router-view />
      </div>

      <NoticeList />
    </section>
  </div>
</template>

<style>
* {
  box-sizing: border-box;
}

html,
body,
#app {
  width: 100%;
  height: 100%;
  margin: 0;
}

body {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  background: transparent;
  overflow: hidden;
}

.app-shell {
  width: 100vw;
  height: 100vh;
  background: #f6f7f7;
  border: 0.0625rem solid #d8dedd;
  border-radius: 1.5rem;
  overflow: hidden;
  display: grid;
  grid-template-columns: 4.1875rem 1fr;
}

.left-nav {
  border-right: 0.0625rem solid #e2e7e6;
  background: #ffffff;
}

.right-panel {
  position: relative;
  height: 100vh;
  display: flex;
  flex-direction: column;
  min-width: 0;
  background: #f9fbfc;
}

.window-titlebar {
  height: 2.5rem;
  border-bottom: 0.0625rem solid #e2e7e6;
  background: #ffffff;
  display: flex;
  align-items: center;
  flex-direction: row-reverse;
  padding: 0 0.625rem 0 0.875rem;
  user-select: none;
  cursor: grab;
}

.window-title {
  font-size: 0.8125rem;
  color: #5a6463;
}

.window-actions {
  display: flex;
  align-items: center;
}

.action-btn {
  width: 1.875rem;
  height: 1.875rem;
  border: 0;
  border-radius: 0.5rem;
  background: transparent;
  color: #576160;
  font-size: 0.875rem;
  cursor: pointer;
}

.close-btn:hover {
  background: #e81123;
  color: #ffffff;
}

.right-content {
  flex: 1;
  min-height: 0;
  background: #f9fbfc;
}
</style>

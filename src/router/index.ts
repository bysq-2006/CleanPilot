import { createRouter, createWebHashHistory } from "vue-router";
import ChatRecordsView from "../pages/ChatRecordsView.vue";
import MainPanelView from "../pages/DiskCleanupView.vue";
import SettingsView from "../pages/settings/SettingsView.vue";

const routes = [
  {
    path: "/",
    name: "main-panel",
    component: MainPanelView,
  },
  {
    path: "/chat-records",
    name: "chat-records",
    component: ChatRecordsView,
  },
  {
    path: "/settings",
    name: "settings",
    component: SettingsView,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;


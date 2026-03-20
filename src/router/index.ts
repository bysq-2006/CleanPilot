import { createRouter, createWebHashHistory } from "vue-router";
import MainPanelView from "../pages/DiskCleanupView.vue";
import SettingsView from "../pages/settings/SettingsView.vue";

const routes = [
  {
    path: "/",
    name: "main-panel",
    component: MainPanelView,
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


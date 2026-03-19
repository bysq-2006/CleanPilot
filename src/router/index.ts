import { createRouter, createWebHashHistory } from "vue-router";
import MainPanelView from "../pages/MainPanelView.vue";
import SettingsView from "../pages/SettingsView.vue";

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


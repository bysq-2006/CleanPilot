import { createRouter, createWebHashHistory } from "vue-router";
import MainPanelView from "../pages/MainPanelView.vue";

const routes = [
  {
    path: "/",
    name: "main-panel",
    component: MainPanelView,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;


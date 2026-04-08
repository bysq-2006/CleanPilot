import { createRouter, createWebHashHistory } from "vue-router";
import ChatRecordsView from "../pages/ChatRecordsView.vue";
import ConversationView from "../pages/ConversationView.vue";
import NewChatView from "../pages/NewChatView.vue";
import StorageBoxView from "../pages/StorageBoxView.vue";
import SettingsView from "../pages/settings/SettingsView.vue";

const routes = [
  {
    path: "/",
    name: "conversation",
    component: ConversationView,
  },
  {
    path: "/new-chat",
    name: "new-chat",
    component: NewChatView,
  },
  {
    path: "/chat-records",
    name: "chat-records",
    component: ChatRecordsView,
  },
  {
    path: "/storage-box",
    name: "storage-box",
    component: StorageBoxView,
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

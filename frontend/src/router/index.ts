import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import DashboardLayout from '../layouts/DashboardLayout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../pages/HomeView.vue"),
    },
    {
      path: "/about",
      name: "about",
      component: () => import("../pages/AboutView.vue"),
    },
  ],
});

export default router;

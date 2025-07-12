import { createRouter, createWebHistory } from "vue-router";

// 导入页面组件
import Dashboard from "./views/Dashboard/index.vue";
import ProxyController from "./views/ProxyController/index.vue";
import PacketDetailWindow from "./views/PacketWindows/index.vue";
import SystemPermissions from "./views/SystemPermissions/index.vue";
import SystemLogs from "./views/SystemLogs/index.vue";
import BiDataMonitor from "./views/BiDataMonitor/index.vue";

// 路由配置
const routes = [
  {
    path: "/",
    name: "Dashboard",
    component: Dashboard
  },
  {
    path: "/proxy",
    name: "ProxyController",
    component: ProxyController
  },
  {
    path: "/packet-detail/:id",
    name: "PacketDetail",
    component: PacketDetailWindow,
  },
  {
    path: "/permissions",
    name: "SystemPermissions",
    component: SystemPermissions
  },
  {
    path: "/logs",
    name: "SystemLogs",
    component: SystemLogs
  },
  {
    path: "/dashboard",
    name: "DashboardPage",
    component: Dashboard
  },
  {
    path: "/bi-monitor",
    name: "BiDataMonitor", 
    component: BiDataMonitor
  }
];

// 创建路由实例
const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;

import { devtools } from '@vue/devtools'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'
import router from "./router.ts";
import './assets/main.css'
import { attachConsole } from '@tauri-apps/plugin-log'

if (process.env.NODE_ENV === 'development') {
  devtools.connect('http://localhost', 8098)
}

// 初始化日志系统 - 将 Rust 日志输出到浏览器控制台
attachConsole().then(() => {
  console.log('🔗 Tauri 日志系统已连接，Rust 日志将显示在控制台中')
}).catch(err => {
  console.error('❌ 连接 Tauri 日志系统失败:', err)
})

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)
app.mount('#app')

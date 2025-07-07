import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

const versionString =
  import.meta.env.MODE === 'development' ? `${import.meta.env.VITE_APP_VERSION}-dev` : import.meta.env.VITE_APP_VERSION

export interface AppState {
  debug: boolean;
  version: string;
  isInitialized: boolean;
  name: string;
  startTime: number;
}

export const useAppStore = defineStore('app', () => {
  // 状态
  const debug = ref(import.meta.env.MODE === 'development')
  const version = ref(versionString)
  const isInitialized = ref(false)
  const name = ref(import.meta.env.VITE_APP_NAME)
  const startTime = ref(0)
  const uptimeTimer = ref<number | null>(null)
  const currentUptime = ref('未启动')

  // 计算属性
  const isReady = computed(() => !isInitialized.value)

  const storeGreet = computed(() => {
    if (name.value.length > 0) {
      return `Greetings from Pinia store, ${name.value}!`
    }
    return ''
  })

  const uptime = computed(() => currentUptime.value)

  // 方法
  const updateUptime = () => {
    if (!startTime.value) {
      currentUptime.value = '未启动'
      return
    }

    const now = Date.now()
    const diff = now - startTime.value

    const seconds = Math.floor(diff / 1000)
    const minutes = Math.floor(seconds / 60)
    const hours = Math.floor(minutes / 60)
    const days = Math.floor(hours / 24)

    if (days > 0) {
      currentUptime.value = `${days}天${hours % 24}小时${minutes % 60}分钟`
    } else if (hours > 0) {
      currentUptime.value = `${hours}小时${minutes % 60}分钟${seconds % 60}秒`
    } else if (minutes > 0) {
      currentUptime.value = `${minutes}分钟${seconds % 60}秒`
    } else {
      currentUptime.value = `${seconds}秒`
    }
  }

  const startUptimeTimer = () => {
    // 清理已存在的定时器
    if (uptimeTimer.value !== null) {
      window.clearInterval(uptimeTimer.value)
    }

    // 立即更新一次
    updateUptime()

    // 设置定时器每秒更新
    uptimeTimer.value = window.setInterval(() => {
      updateUptime()
    }, 1000)
  }

  const stopUptimeTimer = () => {
    if (uptimeTimer.value !== null) {
      window.clearInterval(uptimeTimer.value)
      uptimeTimer.value = null
    }
  }

  const initialize = () => {
    console.log('🚀 开始初始化App Store...')
    isInitialized.value = true
    startTime.value = Date.now()
    startUptimeTimer()
    console.log('✅ App Store初始化完成')
  }

  return {
    // 状态
    debug,
    version,
    isInitialized,
    name,
    startTime,
    currentUptime,

    // 计算属性
    isReady,
    storeGreet,
    uptime,

    // 方法
    initialize,
    startUptimeTimer,
    stopUptimeTimer,
    updateUptime,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot))
}

<template>
  <div class="w-[1200px] h-[800px] mx-auto bg-slate-900 text-white font-sans overflow-hidden">
    <!-- 顶部导航栏 -->
    <header class="h-[60px] bg-gradient-to-r from-slate-800 to-slate-700 border-b border-slate-600 flex items-center justify-between px-6">
      <!-- 左侧返回按钮和标题 -->
      <div class="flex items-center space-x-4">
        <button @click="goBack"
                class="flex items-center space-x-2 px-3 py-2 bg-slate-700 hover:bg-slate-600 rounded-lg transition-colors duration-200">
          <span class="text-lg">←</span>
          <span class="text-sm">返回</span>
        </button>
        <div>
          <h1 class="text-lg font-bold flex items-center gap-2">
            <span>📋</span>
            <span>系统运行日志</span>
          </h1>
          <p class="text-xs text-slate-400">System Runtime Logs</p>
        </div>
      </div>

      <!-- 右侧状态信息 -->
      <div class="flex items-center space-x-4">
        <div class="text-sm text-slate-300">
          系统运行时长: {{ systemRuntime }}
        </div>
        <div class="text-sm text-slate-300">
          {{ currentTime }}
        </div>
      </div>
    </header>

    <!-- 日志控制面板 -->
    <section class="h-[80px] bg-slate-800 border-b border-slate-600 p-4">
      <div class="flex items-center justify-between">
        <!-- 左侧控制按钮 -->
        <div class="flex items-center space-x-3">
          <button @click="refreshLogs"
                  class="flex items-center gap-2 px-4 py-2 bg-blue-500/20 text-blue-300 border border-blue-500/30 rounded-md hover:bg-blue-500/30 transition-all duration-200">
            <span class="text-sm">🔄</span>
            <span>刷新日志</span>
          </button>

          <button @click="clearLogs"
                  class="flex items-center gap-2 px-4 py-2 bg-red-500/20 text-red-300 border border-red-500/30 rounded-md hover:bg-red-500/30 transition-all duration-200">
            <span class="text-sm">🗑️</span>
            <span>清空显示</span>
          </button>

          <button @click="toggleAutoScroll"
                  :class="[
                    'flex items-center gap-2 px-4 py-2 border rounded-md transition-all duration-200',
                    autoScroll
                      ? 'bg-green-500/20 text-green-300 border-green-500/30 hover:bg-green-500/30'
                      : 'bg-gray-500/20 text-gray-300 border-gray-500/30 hover:bg-gray-500/30'
                  ]">
            <span class="text-sm">📜</span>
            <span>{{ autoScroll ? '停止自动刷新' : '开启自动刷新' }}</span>
          </button>
        </div>

        <!-- 右侧过滤器 -->
        <div class="flex items-center space-x-3">
          <div class="flex items-center space-x-2">
            <label class="text-xs text-slate-400">日志级别:</label>
            <select v-model="selectedLogLevel"
                    class="px-3 py-1 bg-slate-700 text-slate-200 border border-slate-600 rounded-md text-xs">
              <option value="all">全部</option>
              <option value="error">Error</option>
              <option value="warn">Warn</option>
              <option value="info">Info</option>
              <option value="debug">Debug</option>
            </select>
          </div>

          <div class="flex items-center space-x-2">
            <label class="text-xs text-slate-400">搜索:</label>
            <input v-model="searchKeyword"
                   type="text"
                   placeholder="搜索日志内容..."
                   class="px-3 py-1 bg-slate-700 text-slate-200 border border-slate-600 rounded-md text-xs w-48">
          </div>
        </div>
      </div>
    </section>

    <!-- 日志内容区域 -->
    <section class="h-[570px] bg-slate-900 p-4">
      <div class="h-full bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg overflow-hidden">
        <!-- 日志统计信息 -->
        <div class="bg-slate-800/60 border-b border-blue-500/20 px-4 py-2">
          <div class="flex items-center justify-between text-xs text-slate-400">
            <div class="flex items-center space-x-4">
              <span>总计: {{ filteredLogs.length }} 条日志</span>
              <span class="text-red-400">Error: {{ getLogCount('error') }}</span>
              <span class="text-yellow-400">Warn: {{ getLogCount('warn') }}</span>
              <span class="text-blue-400">Info: {{ getLogCount('info') }}</span>
              <span class="text-purple-400">Debug: {{ getLogCount('debug') }}</span>
            </div>
            <div>
              <span>最后更新: {{ lastUpdateTime }}</span>
            </div>
          </div>
        </div>

        <!-- 日志列表 -->
        <div ref="logContainer"
             class="h-[calc(100%-40px)] overflow-y-auto font-mono text-xs leading-relaxed">
          <div v-if="filteredLogs.length === 0"
               class="flex flex-col items-center justify-center h-full text-center">
            <div class="text-6xl mb-4 opacity-50">📋</div>
            <div class="text-slate-400 text-lg font-medium mb-2">暂无日志记录</div>
            <div class="text-slate-500 text-sm">系统日志将在这里显示</div>
          </div>

          <div v-else>
            <div v-for="(log, index) in filteredLogs"
                 :key="index"
                 :class="[
                   'px-4 py-2 border-b border-slate-800/50 hover:bg-slate-800/30 transition-colors duration-200',
                   getLogLevelClass(log.level)
                 ]">
              <div class="flex items-start space-x-3">
                <!-- 时间戳 -->
                <div class="text-slate-500 w-20 flex-shrink-0">
                  {{ formatLogTime(log.timestamp) }}
                </div>

                <!-- 日志级别图标 -->
                <div class="w-6 flex-shrink-0">
                  <span :class="getLogLevelIcon(log.level)">{{ getLogLevelIconText(log.level) }}</span>
                </div>

                <!-- 日志内容 -->
                <div class="flex-1 break-all">
                  <div class="text-slate-200">{{ log.message }}</div>
                  <div v-if="log.file && log.line" class="text-slate-500 text-xs mt-1">
                    {{ log.file }}:{{ log.line }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- 底部状态栏 -->
    <section class="h-[60px] bg-slate-800 border-t border-slate-600 px-6 flex items-center justify-between">
      <div class="flex items-center space-x-6">
        <div class="flex items-center space-x-2">
          <div class="w-3 h-3 rounded-full bg-green-500"></div>
          <span class="text-sm text-slate-300">日志系统运行正常</span>
        </div>
        <div class="text-sm text-slate-400">
          日志文件位置: {{ getLogFilePath() }}
        </div>
      </div>

      <div class="text-sm text-slate-400">
        © 重庆秫米科技技术有限公司 {{ new Date().getFullYear() }}
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()

// 响应式数据
const currentTime = ref('')
const systemRuntime = ref('2天 15小时 32分钟')
const lastUpdateTime = ref('')
const autoScroll = ref(true)
const selectedLogLevel = ref('all')
const searchKeyword = ref('')
const logContainer = ref<HTMLElement>()

// 日志数据结构
interface LogEntry {
  timestamp: number
  level: 'error' | 'warn' | 'info' | 'debug'
  message: string
  file?: string
  line?: number
}

const logs = ref<LogEntry[]>([])

// 计算属性
const filteredLogs = computed(() => {
  let filtered = logs.value

  // 按日志级别过滤
  if (selectedLogLevel.value !== 'all') {
    filtered = filtered.filter(log => log.level === selectedLogLevel.value)
  }

  // 按关键词过滤
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    filtered = filtered.filter(log =>
      log.message.toLowerCase().includes(keyword) ||
      (log.file && log.file.toLowerCase().includes(keyword))
    )
  }

  return filtered.sort((a, b) => a.timestamp - b.timestamp)
})

// 定时器
let timeTimer: NodeJS.Timeout | null = null
let refreshTimer: NodeJS.Timeout | null = null

// 方法
const goBack = () => {
  router.push('/dashboard')
}

const updateTime = () => {
  const now = new Date()
  currentTime.value = now.toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
  lastUpdateTime.value = now.toLocaleTimeString('zh-CN')
}

const refreshLogs = async () => {
  try {
    console.log('正在获取系统日志...')

    // 调用 Tauri 命令获取系统日志
    const newLogs = await invoke('get_system_logs') as LogEntry[]
    logs.value = newLogs

        console.log(`成功获取 ${newLogs.length} 条日志`)

    // 日志加载后总是滚动到底部显示最新日志
    await scrollToBottom()
  } catch (error) {
    console.error('刷新日志失败:', error)
    // 显示错误提示
    logs.value = [{
      timestamp: Date.now(),
      level: 'error',
      message: `获取日志失败: ${error}`,
      file: undefined,
      line: undefined
    }]
  }
}

const clearLogs = () => {
  logs.value = []
}

const toggleAutoScroll = () => {
  autoScroll.value = !autoScroll.value
  console.log(autoScroll.value ? '已开启自动刷新' : '已停止自动刷新')
}

const scrollToBottom = async () => {
  await nextTick()
  if (logContainer.value) {
    // 延迟一下确保DOM已更新
    setTimeout(() => {
      if (logContainer.value) {
        logContainer.value.scrollTop = logContainer.value.scrollHeight
      }
    }, 100)
  }
}

const formatLogTime = (timestamp: number): string => {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const getLogLevelClass = (level: string): string => {
  const classes: Record<string, string> = {
    'error': 'border-l-4 border-red-500 bg-red-500/5',
    'warn': 'border-l-4 border-yellow-500 bg-yellow-500/5',
    'info': 'border-l-4 border-blue-500 bg-blue-500/5',
    'debug': 'border-l-4 border-purple-500 bg-purple-500/5'
  }
  return classes[level] || 'border-l-4 border-gray-500 bg-gray-500/5'
}

const getLogLevelIcon = (level: string): string => {
  const classes: Record<string, string> = {
    'error': 'text-red-400',
    'warn': 'text-yellow-400',
    'info': 'text-blue-400',
    'debug': 'text-purple-400'
  }
  return classes[level] || 'text-gray-400'
}

const getLogLevelIconText = (level: string): string => {
  const icons: Record<string, string> = {
    'error': '❌',
    'warn': '⚠️',
    'info': 'ℹ️',
    'debug': '🐛'
  }
  return icons[level] || '📝'
}

const getLogCount = (level: string): number => {
  return logs.value.filter(log => log.level === level).length
}

const getLogFilePath = (): string => {
  const platform = navigator.platform.toLowerCase()
  if (platform.includes('mac')) {
    return '~/Library/Logs/com.big-data-rpa-v4.my/app_logs.log'
  } else if (platform.includes('win')) {
    return '%LOCALAPPDATA%\\com.big-data-rpa-v4.my\\logs\\app_logs.log'
  } else {
    return '~/.local/share/com.big-data-rpa-v4.my/logs/app_logs.log'
  }
}



onMounted(() => {
  updateTime()
  timeTimer = setInterval(updateTime, 1000)

  // 初始化时加载日志
  refreshLogs()

  // 每30秒自动刷新日志
  refreshTimer = setInterval(async () => {
    if (autoScroll.value) {
      await refreshLogs()
    }
  }, 30000)
})

onUnmounted(() => {
  if (timeTimer) clearInterval(timeTimer)
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>

<style scoped>
/* 自定义滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(30, 41, 59, 0.5);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.6), rgba(6, 182, 212, 0.6));
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.8), rgba(6, 182, 212, 0.8));
}

/* 日志条目动画 */
.log-entry-enter-active {
  transition: all 0.3s ease;
}

.log-entry-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}
</style>

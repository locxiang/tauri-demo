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
          系统运行时长: {{ appStore.uptime }}
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

          <button @click="startRealTimeStream"
                  :class="[
                    'flex items-center gap-2 px-4 py-2 border rounded-md transition-all duration-200',
                    isStreaming
                      ? 'bg-green-500/20 text-green-300 border-green-500/30 hover:bg-green-500/30'
                      : 'bg-blue-500/20 text-blue-300 border-blue-500/30 hover:bg-blue-500/30'
                  ]">
            <span class="text-sm">{{ isStreaming ? '📡' : '🔌' }}</span>
            <span>{{ isStreaming ? '停止实时流' : '开启实时流' }}</span>
          </button>

          <button @click="toggleAutoScroll"
                  :class="[
                    'flex items-center gap-2 px-4 py-2 border rounded-md transition-all duration-200',
                    autoScroll
                      ? 'bg-green-500/20 text-green-300 border-green-500/30 hover:bg-green-500/30'
                      : 'bg-gray-500/20 text-gray-300 border-gray-500/30 hover:bg-gray-500/30'
                  ]">
            <span class="text-sm">📜</span>
            <span>{{ autoScroll ? '停止自动滚动' : '开启自动滚动' }}</span>
          </button>

          <button @click="startTestGenerator"
                  class="flex items-center gap-2 px-4 py-2 bg-purple-500/20 text-purple-300 border border-purple-500/30 rounded-md hover:bg-purple-500/30 transition-all duration-200">
            <span class="text-sm">🧪</span>
            <span>启动测试日志</span>
          </button>
        </div>

        <!-- 右侧过滤器 -->
        <div class="flex items-center space-x-3">
          <div class="flex items-center space-x-2">
            <label class="text-xs text-slate-400">日志级别:</label>
            <div class="relative">
              <select v-model="selectedLogLevel"
                      class="px-3 py-1 bg-slate-700 text-slate-200 border border-slate-600 rounded-md text-xs appearance-none cursor-pointer select-dropdown">
                <option value="all">全部</option>
                <option value="error">Error</option>
                <option value="warn">Warn</option>
                <option value="info">Info</option>
                <option value="debug">Debug</option>
                <option value="trace">Trace</option>
              </select>
              <!-- 自定义下拉箭头 -->
              <div class="absolute right-2 top-1/2 transform -translate-y-1/2 pointer-events-none">
                <svg class="w-3 h-3 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
              </div>
            </div>
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
              <span>总计: {{ totalLogCount }} 条日志</span>
              <span class="text-slate-500">内存中: {{ logs.length }} 条</span>
              <span v-if="filteredLogs.length !== logs.length" class="text-blue-400">显示: {{ filteredLogs.length }} 条</span>
              <span class="text-red-400">Error: {{ getLogCount('error') }}</span>
              <span class="text-yellow-400">Warn: {{ getLogCount('warn') }}</span>
              <span class="text-blue-400">Info: {{ getLogCount('info') }}</span>
              <span class="text-purple-400">Debug: {{ getLogCount('debug') }}</span>
              <span class="text-purple-300">Trace: {{ getLogCount('trace') }}</span>
            </div>
            <div class="flex items-center space-x-3">
              <span v-if="isStreaming" class="text-green-400 flex items-center gap-1">
                <span class="animate-pulse">🔴</span>
                <span>实时流</span>
              </span>
              <span v-if="isLoading" class="text-blue-400 flex items-center gap-1">
                <span class="animate-spin">⏳</span>
                <span>加载中...</span>
              </span>
              <span>最后更新: {{ lastUpdateTime }}</span>
            </div>
          </div>
        </div>

        <!-- 日志列表 -->
        <div ref="logContainer"
             class="h-[calc(100%-40px)] overflow-y-auto font-mono text-xs leading-relaxed">
          <!-- 无日志显示 -->
          <div v-if="logs.length === 0"
               class="flex flex-col items-center justify-center h-full text-center">
            <div class="text-6xl mb-4 opacity-50">📋</div>
            <div class="text-slate-400 text-lg font-medium mb-2">暂无日志记录</div>
            <div class="text-slate-500 text-sm">系统日志将在这里显示</div>
            <div v-if="isLoading" class="text-blue-400 text-sm mt-2">正在加载日志...</div>
            <div v-if="errorMessage" class="text-red-400 text-sm mt-2">{{ errorMessage }}</div>
            <div class="text-slate-400 text-sm mt-2">
              <div>调试信息:</div>
              <div>• 总产生日志: {{ totalLogCount }}</div>
              <div>• 内存中日志: {{ logs.length }}</div>
              <div>• 实时流状态: {{ isStreaming ? '开启' : '关闭' }}</div>
            </div>
          </div>

          <!-- 有日志但被过滤掉了 -->
          <div v-else-if="filteredLogs.length === 0"
               class="flex flex-col items-center justify-center h-full text-center">
            <div class="text-6xl mb-4 opacity-50">🔍</div>
            <div class="text-slate-400 text-lg font-medium mb-2">没有符合条件的日志</div>
            <div class="text-slate-500 text-sm">当前过滤器隐藏了所有日志</div>
            <div class="text-slate-400 text-sm mt-2">
              <div>调试信息:</div>
              <div>• 总产生日志: {{ totalLogCount }}</div>
              <div>• 内存中日志: {{ logs.length }}</div>
              <div>• 过滤后显示: {{ filteredLogs.length }}</div>
              <div>• 当前过滤器: 级别={{ selectedLogLevel }}, 搜索={{ searchKeyword || '无' }}</div>
            </div>
          </div>

          <!-- 显示日志 -->
          <div v-else>
            <div v-for="(logEntry, index) in filteredLogs.slice(-100)"
                 :key="index"
                 :class="[
                   'px-4 py-2 border-b border-slate-800/50 hover:bg-slate-800/30 transition-colors duration-200',
                   getLogLevelClass(logEntry.level)
                 ]">
              <div class="flex items-start space-x-3">
                <!-- 时间戳 -->
                <div class="text-slate-500 w-20 flex-shrink-0" :title="formatLogTimestamp(logEntry.timestamp)">
                  {{ formatLogTime(logEntry.timestamp) }}
                </div>

                <!-- 日志级别图标 -->
                <div class="w-6 flex-shrink-0">
                  <span :class="getLogLevelIcon(logEntry.level)">{{ getLogLevelIconText(logEntry.level) }}</span>
                </div>

                <!-- 日志内容 -->
                <div class="flex-1 break-all">
                  <div class="text-slate-200">{{ logEntry.message }}</div>
                  <div class="text-slate-500 text-xs mt-1 flex items-center gap-2">
                    <span v-if="logEntry.module" class="inline-flex items-center gap-1">
                      <span class="text-purple-400">🎯</span>
                      <span class="text-purple-300">{{ logEntry.module }}</span>
                    </span>
                    <span v-if="logEntry.file" class="inline-flex items-center gap-1">
                      <span class="text-blue-400">📄</span>
                      <span class="text-blue-300">{{ logEntry.file }}</span>
                    </span>
                    <span v-if="logEntry.line" class="text-slate-400">行号: {{ logEntry.line }}</span>
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
          <div :class="[
            'w-3 h-3 rounded-full transition-colors duration-200',
            isStreaming ? 'bg-green-500 animate-pulse' : 'bg-gray-500'
          ]"></div>
          <span class="text-sm text-slate-300">
            {{ isStreaming ? '日志实时流运行正常' : '日志系统运行正常' }}
          </span>
        </div>
        <div class="flex items-center space-x-2 text-sm text-slate-400">
          <span>💾 内存缓冲区</span>
          <span class="text-slate-500">|</span>
          <span>已存储: {{ logs.length }} 条</span>
          <span class="text-slate-500">|</span>
          <span>容量: {{ maxLogEntries }} 条</span>
          <span class="text-slate-500">|</span>
          <span class="text-xs text-slate-500">满时自动覆盖旧日志</span>
        </div>
      </div>

      <div class="text-sm text-slate-400">
        © 重庆秫米科技有限公司 {{ new Date().getFullYear() }}
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '@/stores/appStore'
import { useLogStore } from '@/stores/logStore'

const appStore = useAppStore()
const logStore = useLogStore()
const router = useRouter()

// 响应式数据
const autoScroll = ref(true)
const selectedLogLevel = ref('all')
const searchKeyword = ref('')
const logContainer = ref<HTMLElement>()
const autoRefreshInterval = ref(5000) // 5秒自动刷新间隔
const showRealTimeToggle = ref(true)

// 计算属性
const filteredLogs = computed(() => {
  let filtered = logStore.filteredLogs

  // 在Store过滤基础上进行前端额外过滤
  if (selectedLogLevel.value !== 'all') {
    filtered = filtered.filter(log => log.level === selectedLogLevel.value)
  }

  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    filtered = filtered.filter(log =>
      log.message.toLowerCase().includes(keyword) ||
      (log.module && log.module.toLowerCase().includes(keyword)) ||
      (log.file && log.file.toLowerCase().includes(keyword))
    )
  }

  return filtered
})

const lastUpdateTime = computed(() => logStore.lastUpdateTime)
const isLoading = computed(() => logStore.isLoading)
const isStreaming = computed(() => logStore.isStreaming)
const logStats = computed(() => logStore.logLevelCounts)
const errorMessage = computed(() => logStore.error)
const logs = computed(() => logStore.logs)
const maxLogEntries = computed(() => logStore.maxLogEntries)
const totalLogCount = computed(() => logStore.totalLogCount)

// 方法
const goBack = () => {
  router.push('/dashboard')
}

const refreshLogs = async () => {
  await logStore.loadRecentLogs(1000)
  if (autoScroll.value) {
    scrollToBottom()
  }
}

const startRealTimeStream = async () => {
  if (isStreaming.value) {
    await logStore.stopLogStream()
    autoScroll.value = false
  } else {
    await logStore.startLogStream()
    autoScroll.value = true
    // 开启实时流后立即滚动到底部
    setTimeout(() => {
      scrollToBottom()
    }, 100)
  }
}

const updateLogFilters = () => {
  const filters: any = {}

  if (selectedLogLevel.value !== 'all') {
    filters.level = selectedLogLevel.value
  }

  if (searchKeyword.value) {
    filters.keywords = [searchKeyword.value]
  }

  logStore.updateFilters(filters)
}

const clearLogs = async () => {
  await logStore.clearLogs()
}

const toggleAutoScroll = () => {
  autoScroll.value = !autoScroll.value
  if (autoScroll.value) {
    scrollToBottom()
  }
  console.log(autoScroll.value ? '已开启自动滚动' : '已停止自动滚动')
}

const scrollToBottom = () => {
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight
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

const formatLogTimestamp = (timestamp: number): string => {
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
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
    'debug': 'border-l-4 border-purple-500 bg-purple-500/5',
    'trace': 'border-l-4 border-pink-500 bg-pink-500/5'
  }
  return classes[level] || 'border-l-4 border-gray-500 bg-gray-500/5'
}

const getLogLevelIcon = (level: string): string => {
  const classes: Record<string, string> = {
    'error': 'text-red-400',
    'warn': 'text-yellow-400',
    'info': 'text-blue-400',
    'debug': 'text-purple-400',
    'trace': 'text-pink-400'
  }
  return classes[level] || 'text-gray-400'
}

const getLogLevelIconText = (level: string): string => {
  const icons: Record<string, string> = {
    'error': '❌',
    'warn': '⚠️',
    'info': 'ℹ️',
    'debug': '🐛',
    'trace': '🔍'
  }
  return icons[level] || '📝'
}

const getLogCount = (level: string): number => {
  // 统计所有日志中该级别的数量，而不是过滤后的
  return logs.value.filter(log => log.level === level).length
}

const startTestGenerator = async () => {
  try {
    await invoke('start_test_log_generator')
    console.log('🧪 测试日志生成器已启动')
  } catch (err) {
    console.error('❌ 启动测试日志生成器失败:', err)
  }
}

// 监听器
watch(filteredLogs, () => {
  if (autoScroll.value) {
    // 延迟滚动以等待DOM更新
    setTimeout(() => {
      scrollToBottom()
    }, 10)
  }
}, { flush: 'post' })

// 同时监听logs变化，确保实时流中的新日志也能触发滚动
watch(() => logStore.logs, () => {
  if (autoScroll.value && isStreaming.value) {
    setTimeout(() => {
      scrollToBottom()
    }, 10)
  }
}, { flush: 'post', deep: true })

watch([selectedLogLevel, searchKeyword], () => {
  updateLogFilters()
}, { deep: true })

onMounted(async () => {
  // 初始化时加载历史日志
  await refreshLogs()

  // 自动启动实时流
  if (showRealTimeToggle.value) {
    await startRealTimeStream()
  }
})

onUnmounted(async () => {
  // 清理资源
  await logStore.cleanup()
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

/* 跨平台下拉框样式 */
.select-dropdown {
  /* 移除默认的下拉箭头 */
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  /* 确保在所有浏览器中都有相同的基础样式 */
  background-color: transparent;
  color: #e2e8f0; /* text-slate-200 */
}

/* 针对不同浏览器的下拉框样式 */
.select-dropdown::-ms-expand {
  display: none; /* IE/Edge */
}

.select-dropdown:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.6); /* border-blue-500/60 */
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2); /* ring-blue-500/20 */
}

/* 下拉选项样式 */
.select-dropdown option {
  background-color: #1e293b; /* slate-800 */
  color: #e2e8f0; /* text-slate-200 */
  padding: 8px 12px;
  border: none;
  outline: none;
}

.select-dropdown option:hover {
  background-color: #334155; /* slate-700 */
}

.select-dropdown option:checked {
  background-color: #3b82f6; /* blue-500 */
  color: white;
}
</style>

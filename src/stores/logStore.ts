import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 日志条目接口 - 更新为匹配后端结构
export interface LogEntry {
  timestamp: number
  level: 'error' | 'warn' | 'info' | 'debug' | 'trace'
  message: string
  module?: string  // 后端使用module而不是target
  file?: string
  line?: number
}

// 日志过滤器接口 - 更新为匹配后端结构
export interface LogFilters {
  level?: string
  module?: string  // 后端使用module而不是targets
  start_time?: number  // 后端使用start_time而不是since
  end_time?: number    // 后端使用end_time而不是until
}

// 日志统计信息 - 更新为匹配后端结构
export interface LogStats {
  total_count: number
  error_count: number
  warn_count: number
  info_count: number
  debug_count: number
  trace_count: number
}

export const useLogStore = defineStore('log', () => {
  // 状态
  const logs = ref<LogEntry[]>([])
  const isStreaming = ref(false)
  const isLoading = ref(false)
  const currentFilters = ref<LogFilters>({})
  const stats = ref<LogStats>({
    total_count: 0,
    error_count: 0,
    warn_count: 0,
    info_count: 0,
    debug_count: 0,
    trace_count: 0
  })
  const maxLogEntries = ref(1000) // 最大保留的日志条目数
  const lastUpdateTime = ref('')
  const error = ref<string | null>(null)
  const totalLogCount = ref(0) // 总日志数量（累计产生的）

  // 事件监听器
  let logStreamListener: any = null

  // 计算属性
  const filteredLogs = computed(() => {
    let filtered = logs.value

    // 按日志级别过滤
    if (currentFilters.value.level) {
      filtered = filtered.filter(log => log.level === currentFilters.value.level)
    }

    // 按模块过滤 - 更新为使用module字段
    if (currentFilters.value.module) {
      filtered = filtered.filter(log =>
        log.module && log.module.includes(currentFilters.value.module!)
      )
    }

    // 按时间范围过滤 - 更新字段名
    if (currentFilters.value.start_time) {
      filtered = filtered.filter(log => log.timestamp >= currentFilters.value.start_time!)
    }

    if (currentFilters.value.end_time) {
      filtered = filtered.filter(log => log.timestamp <= currentFilters.value.end_time!)
    }

    return filtered.sort((a, b) => a.timestamp - b.timestamp)
  })

  const logLevelCounts = computed(() => {
    const counts = {
      error: 0,
      warn: 0,
      info: 0,
      debug: 0,
      trace: 0
    }

    logs.value.forEach(log => {
      if (log.level in counts) {
        counts[log.level as keyof typeof counts]++
      }
    })

    return counts
  })

  // 方法
  const loadRecentLogs = async (limit: number = 1000) => {
    isLoading.value = true
    error.value = null

    try {
      console.log('🔄 加载最近的日志...')
      const recentLogs = await invoke<LogEntry[]>('get_recent_logs', {
        limit
      })

      logs.value = recentLogs
      lastUpdateTime.value = new Date().toLocaleTimeString()

      // 同时获取总日志数量
      try {
        const totalCount = await invoke<number>('get_total_log_count')
        totalLogCount.value = totalCount
      } catch (err) {
        console.warn('获取总日志数量失败:', err)
      }

      console.log(`✅ 成功加载 ${recentLogs.length} 条日志, 总计产生 ${totalLogCount.value} 条`)
    } catch (err) {
      console.error('❌ 加载日志失败:', err)
      error.value = `加载日志失败: ${err}`
    } finally {
      isLoading.value = false
    }
  }

  const startLogStream = async () => {
    if (isStreaming.value) {
      console.log('📡 日志流已经在运行中')
      return
    }

    try {
      console.log('🚀 开始订阅日志流 (批量模式)...')

      // 订阅后端日志流
      await invoke('subscribe_log_stream')

      // 监听批量日志流事件
      logStreamListener = await listen<LogEntry[]>('log-stream-batch', async (event) => {
        const newLogs = event.payload
        
        // 一次性添加新日志到前端
        logs.value.push(...newLogs)

        // 保持最大数量限制
        if (logs.value.length > maxLogEntries.value) {
          logs.value = logs.value.slice(logs.value.length - maxLogEntries.value)
        }

        // 更新总日志数量
        try {
          const totalCount = await invoke<number>('get_total_log_count')
          totalLogCount.value = totalCount
        } catch (err) {
          console.warn('更新总日志数量失败:', err)
        }

        // 更新最后更新时间
        lastUpdateTime.value = new Date().toLocaleTimeString()

        console.log(`📨 收到 ${newLogs.length} 条新日志, 总计 ${totalLogCount.value} 条`)
      })

      isStreaming.value = true
      console.log('✅ 日志流订阅成功 (批量模式)')
    } catch (err) {
      console.error('❌ 订阅日志流失败:', err)
      error.value = `订阅日志流失败: ${err}`
    }
  }

  const stopLogStream = async () => {
    if (!isStreaming.value) {
      console.log('📡 日志流已经停止')
      return
    }

    try {
      console.log('🛑 停止日志流订阅...')

      // 取消前端事件监听
      if (logStreamListener) {
        logStreamListener()
        logStreamListener = null
      }

      isStreaming.value = false
      console.log('✅ 日志流订阅已停止')
    } catch (err) {
      console.error('❌ 停止日志流失败:', err)
      error.value = `停止日志流失败: ${err}`
    }
  }

  const clearLogs = async () => {
    try {
      console.log('🗑️ 清空日志...')

      // 清空后端日志缓冲区
      await invoke('clear_logs')

      // 清空前端日志
      logs.value = []
      lastUpdateTime.value = new Date().toLocaleTimeString()

      console.log('✅ 日志已清空')
    } catch (err) {
      console.error('❌ 清空日志失败:', err)
      error.value = `清空日志失败: ${err}`
    }
  }

  const updateFilters = (filters: LogFilters) => {
    currentFilters.value = { ...filters }
    console.log('🔍 更新日志过滤器:', filters)
  }

  const getLogStats = async () => {
    try {
      const logStats = await invoke<LogStats>('get_log_stats')
      stats.value = logStats
      return logStats
    } catch (err) {
      console.error('❌ 获取日志统计失败:', err)
      error.value = `获取日志统计失败: ${err}`
      return null
    }
  }

  const cleanup = async () => {
    if (isStreaming.value) {
      await stopLogStream()
    }
  }

  return {
    // 状态
    logs,
    isStreaming,
    isLoading,
    currentFilters,
    stats,
    maxLogEntries,
    lastUpdateTime,
    error,
    totalLogCount,

    // 计算属性
    filteredLogs,
    logLevelCounts,

    // 方法
    loadRecentLogs,
    startLogStream,
    stopLogStream,
    clearLogs,
    updateFilters,
    getLogStats,
    cleanup
  }
})
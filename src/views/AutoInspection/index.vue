<template>
  <div class="h-screen overflow-hidden relative text-slate-200 bg-gradient-to-br from-slate-900 via-slate-800 to-slate-700">

    <!-- 顶部导航栏 -->
    <div class="h-15 flex items-center justify-between px-6 bg-gradient-to-r from-slate-900/95 to-slate-800/90 border-b border-green-500/25 backdrop-blur-2xl relative z-[100] shadow-2xl">

      <div class="flex items-center gap-6">
        <button
          @click="goBack"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-green-500/10 to-cyan-500/10 border border-green-500/30 rounded-md text-slate-200 hover:from-green-500/20 hover:to-cyan-500/20 hover:border-green-500/50 transition-all duration-300 relative overflow-hidden group"
        >
          <div class="text-lg font-bold">←</div>
          <span class="text-sm">返回控制台</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>

        <div class="flex items-center gap-3">
          <div class="text-2xl animate-icon-glow">📈</div>
          <h1 class="text-xl font-bold text-slate-200 font-mono tracking-wide">自动化巡检体证指标</h1>
        </div>
      </div>

      <div class="flex items-center gap-4">
        <!-- 监控状态 -->
        <div class="flex items-center gap-3 px-4 py-2 bg-gradient-to-r from-slate-900/80 to-slate-800/60 border border-green-500/30 rounded-md backdrop-blur-lg">
          <div :class="['w-3 h-3 rounded-full relative border-2 shadow-lg',
                        isMonitoring ? 'bg-green-500 border-green-500/50 shadow-green-500/50' : 'bg-orange-500 border-orange-500/50 shadow-orange-500/50']">
          </div>
          <span class="text-sm font-semibold">{{ isMonitoring ? '巡检中' : '待配置' }}</span>
        </div>

        <!-- 刷新按钮 -->
        <button
          @click="refreshData"
          :disabled="isLoading"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-green-500/10 to-emerald-500/10 border border-green-500/30 rounded-md text-slate-200 hover:from-green-500/20 hover:to-emerald-500/20 hover:border-green-500/50 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed relative overflow-hidden group"
        >
          <span class="text-lg" :class="{ 'animate-spin': isLoading }">🔄</span>
          <span class="text-sm">{{ isLoading ? '刷新中...' : '刷新数据' }}</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>

        <!-- 导出报告按钮 -->
        <button
          @click="exportReport"
          :disabled="isLoading"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-500/10 to-indigo-500/10 border border-blue-500/30 rounded-md text-slate-200 hover:from-blue-500/20 hover:to-indigo-500/20 hover:border-blue-500/50 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed relative overflow-hidden group"
        >
          <span class="text-lg">📄</span>
          <span class="text-sm">导出报告</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="h-[calc(100vh-60px)] p-6 relative z-10 overflow-hidden">
      <!-- 错误提示 -->
      <div v-if="error" class="bg-gradient-to-br from-red-500/20 to-pink-500/20 backdrop-blur-2xl border border-red-500/30 rounded-xl p-4 mb-6 shadow-2xl flex items-center gap-4">
        <div class="text-2xl animate-icon-pulse">⚠️</div>
        <div class="flex flex-col flex-1">
          <span class="font-semibold text-red-300">发生错误</span>
          <span class="text-red-400 text-sm">{{ error }}</span>
        </div>
        <button
          @click="error = ''"
          class="flex items-center justify-center w-8 h-8 bg-red-500/20 border border-red-500/30 rounded-lg text-red-300 hover:bg-red-500/30 hover:border-red-500/50 transition-all duration-300"
          title="关闭错误提示"
        >
          <span class="text-lg">×</span>
        </button>
      </div>

      <!-- 统计数据面板 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-green-500/20 rounded-xl p-6 mb-6 shadow-2xl hover:shadow-green-500/10 transition-all duration-300">
        <div class="mb-4">
          <div class="flex items-center gap-3 relative">
            <div class="text-xl">📊</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">巡检统计</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-green-500/60 via-cyan-500/60 to-transparent ml-4"></div>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <!-- 上一次更新事件 -->
          <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/20 rounded-lg p-4 relative overflow-hidden group">
            <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-blue-500/80 to-transparent"></div>
            <div class="flex items-center gap-4">
              <div class="text-3xl">⏰</div>
              <div class="flex-1">
                <div class="text-sm font-medium text-slate-400 mb-1">上一次更新事件</div>
                <div class="text-lg font-bold text-blue-400 font-mono">{{ lastUpdateTime }}</div>
              </div>
            </div>
          </div>

          <!-- 总指标数 -->
          <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-green-500/20 rounded-lg p-4 relative overflow-hidden group">
            <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-green-500/80 to-transparent"></div>
            <div class="flex items-center gap-4">
              <div class="text-3xl">📈</div>
              <div class="flex-1">
                <div class="text-sm font-medium text-slate-400 mb-1">总指标数</div>
                <div class="text-lg font-bold text-green-400 font-mono">{{ totalIndicators.toLocaleString() }}</div>
              </div>
            </div>
          </div>

          <!-- 异常指标数 -->
          <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-red-500/20 rounded-lg p-4 relative overflow-hidden group">
            <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-red-500/80 to-transparent"></div>
            <div class="flex items-center gap-4">
              <div class="text-3xl">🚨</div>
              <div class="flex-1">
                <div class="text-sm font-medium text-slate-400 mb-1">异常指标数</div>
                <div class="text-lg font-bold text-red-400 font-mono">{{ abnormalIndicators.toLocaleString() }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 异常指标列表 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-green-500/20 rounded-xl shadow-2xl hover:shadow-green-500/10 transition-all duration-300 relative overflow-hidden">

        <div class="p-6 border-b border-green-500/20">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="text-2xl">🚨</div>
              <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">异常指标详情</h2>
            </div>
            <div class="flex items-center gap-4">
              <!-- 异常统计 -->
              <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-red-500/20 to-pink-500/20 border border-red-500/30 rounded-full">
                <div class="w-2 h-2 bg-red-400 rounded-full"></div>
                <span class="text-sm font-semibold text-red-400">异常: {{ abnormalIndicators }}</span>
              </div>
              <!-- 正常统计 -->
              <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-green-500/20 to-emerald-500/20 border border-green-500/30 rounded-full">
                <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                <span class="text-sm font-semibold text-green-400">正常: {{ totalIndicators - abnormalIndicators }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 列表内容 -->
        <div class="overflow-auto max-h-[calc(100vh-400px)]">
          <!-- 加载状态 -->
          <div v-if="isLoading" class="p-8 text-center">
            <div class="inline-flex items-center gap-3">
              <svg class="animate-spin h-6 w-6 text-green-500" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <span class="text-slate-300">正在加载数据...</span>
            </div>
          </div>

          <!-- 空数据状态 -->
          <div v-else-if="abnormalDataList.length === 0" class="p-8 text-center">
            <div class="text-6xl mb-4 opacity-60">✅</div>
            <div class="text-slate-400 text-lg">暂无异常指标</div>
            <div class="text-slate-500 text-sm mt-2">所有体证指标运行正常</div>
          </div>

          <!-- 数据表格 -->
          <div v-else class="overflow-x-auto">
            <table class="w-full table-fixed">
              <!-- 表头 -->
              <thead class="bg-gradient-to-r from-slate-800/80 to-slate-700/60 border-b border-slate-600/50">
                <tr>
                  <th class="w-20 px-6 py-4 text-center text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    序号
                  </th>
                  <th class="w-1/4 px-6 py-4 text-left text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    指标名称
                  </th>
                  <th class="w-1/6 px-6 py-4 text-center text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    指标在大屏的页码
                  </th>
                  <th class="w-1/2 px-6 py-4 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">
                    指标异常原因
                  </th>
                </tr>
              </thead>

              <!-- 表体 -->
              <tbody class="divide-y divide-slate-700/50">
                <tr
                  v-for="(item, index) in abnormalDataList"
                  :key="index"
                  class="hover:bg-slate-800/30 transition-all duration-300 relative group"
                >
                  <!-- 序号 -->
                  <td class="w-20 px-6 py-4 text-center border-r border-slate-700/30">
                    <div class="text-sm text-slate-300 font-mono">{{ index + 1 }}</div>
                  </td>

                  <!-- 指标名称 -->
                  <td class="w-1/4 px-6 py-4 border-r border-slate-700/30">
                    <div class="text-sm text-blue-300 font-semibold break-words">{{ item.indicatorName }}</div>
                  </td>

                  <!-- 指标在大屏的页码 -->
                  <td class="w-1/6 px-6 py-4 text-center border-r border-slate-700/30">
                    <div class="text-sm text-cyan-300 font-semibold">{{ item.pageNumber }}</div>
                  </td>

                  <!-- 指标异常原因 -->
                  <td class="w-1/2 px-6 py-4">
                    <div class="text-sm text-red-300 font-semibold break-words">{{ item.abnormalReason }}</div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// 响应式数据
const lastUpdateTime = ref('2024-01-15 14:30:25')
const totalIndicators = ref(1250)
const abnormalIndicators = ref(23)
const isLoading = ref(false)
const error = ref('')

// 监控状态
const isMonitoring = computed(() => true)

// 异常指标数据接口
interface AbnormalIndicator {
  indicatorName: string
  pageNumber: number
  abnormalReason: string
}

// 模拟异常指标数据
const abnormalDataList = ref<AbnormalIndicator[]>([
  {
    indicatorName: 'GDP增长率',
    pageNumber: 1,
    abnormalReason: '数据更新延迟超过24小时，影响实时监控'
  },
  {
    indicatorName: 'CPI指数',
    pageNumber: 2,
    abnormalReason: '数据源异常，无法获取最新数据'
  },
  {
    indicatorName: 'PMI制造业指数',
    pageNumber: 3,
    abnormalReason: '数据格式错误，需要重新处理'
  },
  {
    indicatorName: '社会消费品零售总额',
    pageNumber: 4,
    abnormalReason: '数据缺失，部分区域数据未上报'
  },
  {
    indicatorName: '固定资产投资额',
    pageNumber: 5,
    abnormalReason: '数据异常波动，超出正常范围'
  },
  {
    indicatorName: '进出口总额',
    pageNumber: 6,
    abnormalReason: '数据源连接失败，无法同步'
  },
  {
    indicatorName: '居民消费价格指数',
    pageNumber: 7,
    abnormalReason: '数据计算错误，需要重新校验'
  },
  {
    indicatorName: '工业增加值',
    pageNumber: 8,
    abnormalReason: '数据更新频率异常，影响监控效果'
  },
  {
    indicatorName: '城镇新增就业人数',
    pageNumber: 9,
    abnormalReason: '数据源权限变更，需要重新授权'
  },
  {
    indicatorName: '农村居民人均可支配收入',
    pageNumber: 10,
    abnormalReason: '数据格式不统一，需要标准化处理'
  },
  {
    indicatorName: '城镇居民人均可支配收入',
    pageNumber: 11,
    abnormalReason: '数据质量不达标，存在重复记录'
  },
  {
    indicatorName: '全社会用电量',
    pageNumber: 12,
    abnormalReason: '数据采集设备故障，影响数据准确性'
  },
  {
    indicatorName: '铁路货运量',
    pageNumber: 13,
    abnormalReason: '数据接口异常，无法正常获取'
  },
  {
    indicatorName: '港口货物吞吐量',
    pageNumber: 14,
    abnormalReason: '数据更新机制故障，需要重启服务'
  },
  {
    indicatorName: '商品房销售面积',
    pageNumber: 15,
    abnormalReason: '数据源维护中，暂时无法访问'
  },
  {
    indicatorName: '汽车销量',
    pageNumber: 16,
    abnormalReason: '数据格式变更，需要适配新格式'
  },
  {
    indicatorName: '移动电话用户数',
    pageNumber: 17,
    abnormalReason: '数据加密异常，无法解密处理'
  },
  {
    indicatorName: '互联网用户数',
    pageNumber: 18,
    abnormalReason: '数据备份失败，存在数据丢失风险'
  },
  {
    indicatorName: '金融机构人民币贷款余额',
    pageNumber: 19,
    abnormalReason: '数据同步延迟，影响实时分析'
  },
  {
    indicatorName: '外汇储备',
    pageNumber: 20,
    abnormalReason: '数据源负载过高，响应超时'
  },
  {
    indicatorName: '财政收入',
    pageNumber: 21,
    abnormalReason: '数据格式不兼容，需要转换处理'
  },
  {
    indicatorName: '财政支出',
    pageNumber: 22,
    abnormalReason: '数据源网络异常，连接不稳定'
  },
  {
    indicatorName: '税收收入',
    pageNumber: 23,
    abnormalReason: '数据更新频率过低，影响监控时效性'
  }
])

// 返回控制台
const goBack = () => {
  router.push('/')
}

// 刷新数据
const refreshData = async () => {
  isLoading.value = true
  error.value = ''

  try {
    // 模拟API调用
    await new Promise(resolve => setTimeout(resolve, 2000))

    // 更新数据
    lastUpdateTime.value = new Date().toLocaleString()
    totalIndicators.value = Math.floor(Math.random() * 500) + 1000
    abnormalIndicators.value = Math.floor(Math.random() * 50) + 10

    // 模拟更新异常数据
    abnormalDataList.value = abnormalDataList.value.map((item, index) => ({
      ...item,
      pageNumber: index + 1
    }))

  } catch (err) {
    error.value = '刷新数据失败，请稍后重试'
  } finally {
    isLoading.value = false
  }
}

// 导出报告
const exportReport = () => {
  // 这里可以实现导出报告的逻辑
  console.log('导出巡检报告')
}

// 页面加载时初始化数据
onMounted(() => {
  // 可以在这里加载初始数据
  console.log('自动化巡检体证指标页面已加载')
})
</script>

<style scoped>
/* 页面样式 */
</style>

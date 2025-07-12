<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import PacketTable from './components/PacketTable.vue';
import PacketFilter from './components/PacketFilter.vue';
import { useProxyStore } from '../../stores/proxyStore.ts';
import './animations.css';

// 导入SVG图标
import HttpRequestIcon from '../../assets/icons/http-request.svg';
import HttpResponseIcon from '../../assets/icons/http-response.svg';

const router = useRouter();
const proxyStore = useProxyStore();

// 表格最大化状态
const isTableMaximized = ref(false);

const goBack = () => {
  router.push('/');
};

// 切换表格最大化状态
const toggleTableMaximize = () => {
  console.log('toggleTableMaximize', isTableMaximized.value);
  isTableMaximized.value = !isTableMaximized.value;
};


</script>

<template>
  <div class="h-screen overflow-hidden relative text-slate-200 bg-gradient-to-br from-slate-900 via-slate-800 to-slate-700">
    <!-- 粒子动画背景 -->
    <div class="absolute inset-0 z-0 pointer-events-none particles-bg animate-particle-float"></div>

    <!-- 动态背景网格 -->
    <div class="absolute inset-0 z-[1] pointer-events-none grid-pattern animate-grid-float"></div>
    <div class="absolute inset-0 z-[2] pointer-events-none bg-gradient-to-br from-blue-500/5 via-purple-500/3 to-cyan-500/5 animate-overlay-pulse"></div>

    <!-- 科技装饰元素 -->
    <div class="absolute inset-0 z-[3] pointer-events-none">
      <div class="absolute top-[10%] right-[15%] w-48 h-48 circuit-pattern opacity-60 animate-circuit-glow rotate-[15deg]"></div>
      <div class="absolute bottom-[20%] left-[10%] w-48 h-48 circuit-pattern opacity-60 animate-circuit-glow -rotate-[25deg] animation-delay-2s"></div>
      <div class="absolute top-[20%] left-[25%] w-0.5 h-24 data-stream animate-data-flow"></div>
      <div class="absolute top-[60%] right-[30%] w-0.5 h-24 data-stream animate-data-flow animation-delay-1_5s"></div>
    </div>

    <!-- 顶部导航栏 -->
    <div class="h-15 flex items-center justify-between px-6 bg-gradient-to-r from-slate-900/95 to-slate-800/90 border-b border-blue-500/25 backdrop-blur-2xl relative z-[100] shadow-2xl">
      <!-- 动画扫描线 -->
      <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-blue-500/80 to-transparent animate-header-scan"></div>

      <div class="flex items-center gap-6">
        <button
          @click="goBack"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-500/10 to-cyan-500/10 border border-blue-500/30 rounded-md text-slate-200 hover:from-blue-500/20 hover:to-cyan-500/20 hover:border-blue-500/50 hover:-translate-y-0.5 transition-all duration-300 relative overflow-hidden group"
        >
          <div class="text-lg font-bold">←</div>
          <span class="text-sm">返回控制台</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>

        <div class="flex items-center gap-3">
          <div class="text-2xl animate-icon-glow">📡</div>
          <h1 class="text-xl font-bold text-slate-200 font-mono tracking-wide">网络抓包分析器</h1>
        </div>
      </div>

      <div class="flex items-center gap-4">
        <div class="flex items-center gap-3 px-4 py-2 bg-gradient-to-r from-slate-900/80 to-slate-800/60 border border-blue-500/30 rounded-md backdrop-blur-lg">
          <div :class="['w-3 h-3 rounded-full relative border-2 shadow-lg',
                        proxyStore.captureStatus.running ? 'bg-green-500 border-green-500/50 shadow-green-500/50' : 'bg-red-500 border-red-500/50 shadow-red-500/50']">
            <div v-if="proxyStore.captureStatus.running" class="absolute inset-0 bg-green-500 rounded-full animate-pulse-custom"></div>
            <div v-else class="absolute inset-0 bg-red-500 rounded-full animate-status-blink"></div>
          </div>
          <span class="text-sm font-semibold">{{ proxyStore.captureStatus.running ? '抓包中' : '已停止' }}</span>
        </div>

        <button
          @click="proxyStore.clearPackets"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded-md text-slate-200 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 hover:-translate-y-0.5 transition-all duration-300 relative overflow-hidden group"
        >
          <span class="text-lg">🧹</span>
          <span class="text-sm">清空记录</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="h-[calc(100vh-60px)] overflow-y-auto p-6 relative z-10">
      <!-- 错误提示 -->
      <div v-if="proxyStore.error" class="bg-gradient-to-br from-red-500/20 to-pink-500/20 backdrop-blur-2xl border border-red-500/30 rounded-xl p-4 mb-6 shadow-2xl flex items-center gap-4">
        <div class="text-2xl animate-icon-pulse">⚠️</div>
        <div class="flex flex-col">
          <span class="font-semibold text-red-300">发生错误</span>
          <span class="text-red-400 text-sm">{{ proxyStore.error }}</span>
        </div>
      </div>

      <!-- 控制面板 -->
      <div v-show="!isTableMaximized" class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl p-4 mb-4 shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300">
        <div class="mb-4">
          <div class="flex items-center gap-3 relative">
            <div class="text-xl animate-icon-glow">🎛️</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">抓包控制</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-blue-500/60 via-cyan-500/60 to-transparent ml-4 relative overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-line-scan"></div>
            </div>
          </div>
        </div>

        <div class="flex items-center gap-4">
          <!-- 网络设备选择 - 占50%宽度 -->
          <div class="w-1/2 bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg p-3">
            <label class="block text-sm font-semibold text-slate-200 mb-2">选择网络设备</label>
            <div class="relative">
              <select
                v-model="proxyStore.selectedDevice"
                :disabled="proxyStore.captureStatus.running || proxyStore.isLoading"
                class="w-full px-3 py-2 bg-gradient-to-r from-slate-900/80 to-slate-800/60 border border-blue-500/30 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500/60 focus:ring-2 focus:ring-blue-500/20 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed appearance-none cursor-pointer select-dropdown"
              >
                <option v-if="proxyStore.devices.length === 0" value="" disabled>未找到网络设备</option>
                <option v-for="device in proxyStore.devices" :key="device.name" :value="device.name" class="select-option">
                  {{ device.name }} - {{ device.description }}
                  {{ device.is_loopback ? ' (回环)' : '' }}
                </option>
              </select>
              <!-- 自定义下拉箭头 -->
              <div class="absolute right-3 top-1/2 transform -translate-y-1/2 pointer-events-none">
                <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
              </div>
            </div>
          </div>

          <!-- 控制按钮 - 占剩余50%宽度 -->
          <div class="flex-1 flex items-center gap-3">
            <button
              @click="proxyStore.startCapture"
              :disabled="!proxyStore.selectedDevice || proxyStore.captureStatus.running || proxyStore.isLoading"
              class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-green-500/10 to-emerald-500/10 border border-green-500/30 rounded-lg text-slate-200 hover:from-green-500/20 hover:to-emerald-500/20 hover:border-green-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:translate-y-0 relative overflow-hidden group"
            >
              <span class="text-lg">▶️</span>
              <span class="font-semibold">{{ proxyStore.isLoading ? '启动中...' : '开始抓包' }}</span>
              <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
            </button>

            <button
              @click="proxyStore.stopCapture"
              :disabled="!proxyStore.captureStatus.running || proxyStore.isLoading"
              class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded-lg text-slate-200 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:translate-y-0 relative overflow-hidden group"
            >
              <span class="text-lg">⏹️</span>
              <span class="font-semibold">{{ proxyStore.isLoading ? '停止中...' : '停止抓包' }}</span>
              <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
            </button>
          </div>
        </div>
      </div>

      <!-- 数据包过滤器 -->
      <PacketFilter v-show="!isTableMaximized" />

      <!-- 实时数据包列表 -->
      <div :class="['bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300 relative overflow-hidden',
                   isTableMaximized ? 'mt-0' : '']">
        <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-blue-500/80 to-transparent animate-packet-scan"></div>

        <div class="p-6 border-b border-blue-500/20">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="text-2xl animate-icon-glow">📡</div>
              <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">实时HTTP数据包</h2>
              <div class="flex items-center gap-4">
                <!-- 总数统计 -->
                <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-blue-500/20 to-cyan-500/20 border border-blue-500/30 rounded-full">
                  <div class="w-2 h-2 bg-cyan-400 rounded-full animate-pulse-custom"></div>
                  <span class="text-sm font-semibold text-cyan-400">
                    显示: {{ proxyStore.filteredPacketCount }}
                    <span v-if="proxyStore.filters.enabled" class="text-cyan-500">
                      / {{ proxyStore.packetCount }}
                    </span>
                  </span>
                </div>
                <!-- 请求统计 -->
                <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-cyan-500/20 to-blue-500/20 border border-cyan-500/30 rounded-full">
                  <img :src="HttpRequestIcon" alt="HTTP请求" class="w-4 h-4 request-icon" />
                  <span class="text-sm font-semibold text-cyan-300">请求: {{ proxyStore.requestCount }}</span>
                </div>
                <!-- 响应统计 -->
                <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-green-500/20 to-emerald-500/20 border border-green-500/30 rounded-full">
                  <img :src="HttpResponseIcon" alt="HTTP响应" class="w-4 h-4 response-icon" />
                  <span class="text-sm font-semibold text-green-300">响应: {{ proxyStore.responseCount }}</span>
                </div>
              </div>
            </div>

            <div class="flex items-center gap-4">
              <button
                @click="toggleTableMaximize"
                class="flex items-center gap-2 px-3 py-1.5 bg-gradient-to-r from-indigo-500/10 to-purple-500/10 border border-indigo-500/30 rounded-lg text-slate-200 hover:from-indigo-500/20 hover:to-purple-500/20 hover:border-indigo-500/50 hover:-translate-y-0.5 transition-all duration-300 relative overflow-hidden group"
                :title="isTableMaximized ? '显示控制面板' : '隐藏控制面板'"
              >
                <span class="text-sm">{{ isTableMaximized ? '📤' : '📥' }}</span>
                <span class="text-sm font-medium">{{ isTableMaximized ? '还原' : '最大化' }}</span>
                <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
              </button>
            </div>
          </div>
        </div>

        <PacketTable :packets="proxyStore.filteredPackets" :is-maximized="isTableMaximized" />
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 动画导入已在 script 中处理 */

/* SVG图标样式 */
.request-icon {
  filter: brightness(0) saturate(100%) invert(62%) sepia(98%) saturate(1946%) hue-rotate(158deg) brightness(103%) contrast(88%);
}

.response-icon {
  filter: brightness(0) saturate(100%) invert(48%) sepia(98%) saturate(1295%) hue-rotate(85deg) brightness(98%) contrast(97%);
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

/* 禁用状态 */
.select-dropdown:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: rgba(30, 41, 59, 0.8); /* slate-800 with opacity */
}

/* 确保自定义箭头在禁用时也正确显示 */
.select-dropdown:disabled + div svg {
  opacity: 0.5;
}
</style>

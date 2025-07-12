<template>
  <div class="h-screen overflow-hidden relative text-slate-200 bg-gradient-to-br from-slate-900 via-slate-800 to-slate-700">
    <!-- 粒子动画背景 -->
    <div class="absolute inset-0 z-0 pointer-events-none particles-bg animate-particle-float"></div>

    <!-- 动态背景网格 -->
    <div class="absolute inset-0 z-[1] pointer-events-none grid-pattern animate-grid-float"></div>
    <div class="absolute inset-0 z-[2] pointer-events-none bg-gradient-to-br from-green-500/5 via-purple-500/3 to-cyan-500/5 animate-overlay-pulse"></div>

    <!-- 科技装饰元素 -->
    <div class="absolute inset-0 z-[3] pointer-events-none">
      <div class="absolute top-[10%] right-[15%] w-48 h-48 circuit-pattern opacity-60 animate-circuit-glow rotate-[15deg]"></div>
      <div class="absolute bottom-[20%] left-[10%] w-48 h-48 circuit-pattern opacity-60 animate-circuit-glow -rotate-[25deg] animation-delay-2s"></div>
      <div class="absolute top-[20%] left-[25%] w-0.5 h-24 data-stream animate-data-flow"></div>
      <div class="absolute top-[60%] right-[30%] w-0.5 h-24 data-stream animate-data-flow animation-delay-1_5s"></div>
    </div>

    <!-- 顶部导航栏 -->
    <div class="h-15 flex items-center justify-between px-6 bg-gradient-to-r from-slate-900/95 to-slate-800/90 border-b border-green-500/25 backdrop-blur-2xl relative z-[100] shadow-2xl">
      <!-- 动画扫描线 -->
      <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-green-500/80 to-transparent animate-header-scan"></div>

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
          <div class="text-2xl animate-icon-glow">📊</div>
          <h1 class="text-xl font-bold text-slate-200 font-mono tracking-wide">BI数据异常监控</h1>
        </div>
      </div>

      <div class="flex items-center gap-4">
        <!-- 监控状态 -->
        <div class="flex items-center gap-3 px-4 py-2 bg-gradient-to-r from-slate-900/80 to-slate-800/60 border border-green-500/30 rounded-md backdrop-blur-lg">
          <div :class="['w-3 h-3 rounded-full relative border-2 shadow-lg',
                        isMonitoring ? 'bg-green-500 border-green-500/50 shadow-green-500/50' : 'bg-orange-500 border-orange-500/50 shadow-orange-500/50']">
            <div v-if="isMonitoring" class="absolute inset-0 bg-green-500 rounded-full animate-pulse-custom"></div>
            <div v-else class="absolute inset-0 bg-orange-500 rounded-full animate-status-blink"></div>
          </div>
          <span class="text-sm font-semibold">{{ isMonitoring ? '监控中' : '待配置' }}</span>
        </div>

        <!-- 数据目录按钮 -->
        <button
          @click="selectDataDirectory"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-500/10 to-indigo-500/10 border border-blue-500/30 rounded-md text-slate-200 hover:from-blue-500/20 hover:to-indigo-500/20 hover:border-blue-500/50 transition-all duration-300 relative overflow-hidden group cursor-pointer"
        >
          <span class="text-lg">📁</span>
          <span class="text-sm">{{ dataDirectory ? '更换目录' : '选择数据目录' }}</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>

        <!-- 一键关联按钮 -->
        <button
          @click="associateAllFiles"
          :disabled="!dataDirectory || isAssociating || dataList.length === 0"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-purple-500/10 to-pink-500/10 border border-purple-500/30 rounded-md text-slate-200 hover:from-purple-500/20 hover:to-pink-500/20 hover:border-purple-500/50 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed relative overflow-hidden group cursor-pointer"
        >
          <span class="text-lg">{{ isAssociating ? '🔄' : '🔗' }}</span>
          <span class="text-sm">{{ isAssociating ? '关联中...' : '一键关联' }}</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>

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

        <!-- BI平台查询按钮 -->
        <button
          @click="sendBiQuery"
          :disabled="biPlatformStore.isLoading"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-purple-500/10 to-indigo-500/10 border border-purple-500/30 rounded-md text-slate-200 hover:from-purple-500/20 hover:to-indigo-500/20 hover:border-purple-500/50 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed relative overflow-hidden group"
        >
          <span class="text-lg">📊</span>
          <span class="text-sm">{{ biPlatformStore.isLoading ? '查询中...' : 'BI平台查询' }}</span>
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

      <!-- BI平台查询结果 -->
      <div v-if="biPlatformStore.hasError" class="bg-gradient-to-br from-red-500/20 to-pink-500/20 backdrop-blur-2xl border border-red-500/30 rounded-xl p-4 mb-6 shadow-2xl flex items-center gap-4">
        <div class="text-2xl animate-icon-pulse">⚠️</div>
        <div class="flex flex-col flex-1">
          <span class="font-semibold text-red-300">BI平台查询错误</span>
          <span class="text-red-400 text-sm">{{ biPlatformStore.error }}</span>
        </div>
        <button
          @click="biPlatformStore.clearError"
          class="flex items-center justify-center w-8 h-8 bg-red-500/20 border border-red-500/30 rounded-lg text-red-300 hover:bg-red-500/30 hover:border-red-500/50 transition-all duration-300"
          title="关闭错误提示"
        >
          <span class="text-lg">×</span>
        </button>
      </div>

      <div v-if="biPlatformStore.hasResponse && biPlatformStore.lastResponse?.success" class="bg-gradient-to-br from-green-500/20 to-emerald-500/20 backdrop-blur-2xl border border-green-500/30 rounded-xl p-4 mb-6 shadow-2xl">
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center gap-3">
            <div class="text-2xl animate-icon-glow">✅</div>
            <h3 class="text-lg font-semibold text-green-300">BI平台查询成功</h3>
          </div>
          <button
            @click="biPlatformStore.clearResponse"
            class="flex items-center justify-center w-8 h-8 bg-green-500/20 border border-green-500/30 rounded-lg text-green-300 hover:bg-green-500/30 hover:border-green-500/50 transition-all duration-300"
            title="关闭成功提示"
          >
            <span class="text-lg">×</span>
          </button>
        </div>
        <div class="text-green-400 text-sm">
          <p>查询时间: {{ new Date().toLocaleString() }}</p>
          <p v-if="biPlatformStore.lastResponse?.message">消息: {{ biPlatformStore.lastResponse.message }}</p>
        </div>
      </div>

      <!-- 数据目录状态 -->
      <div v-if="dataDirectory" class="bg-gradient-to-br from-blue-500/20 to-cyan-500/20 backdrop-blur-2xl border border-blue-500/30 rounded-xl p-4 mb-6 shadow-2xl flex items-center gap-4">
        <div class="text-2xl animate-icon-glow">📂</div>
        <div class="flex flex-col flex-1">
          <span class="font-semibold text-blue-300">数据目录</span>
          <span class="text-blue-400 text-sm font-mono">{{ dataDirectory }}</span>
        </div>
        <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-green-500/20 to-emerald-500/20 border border-green-500/30 rounded-full">
          <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse-custom"></div>
          <span class="text-sm font-semibold text-green-400">已配置</span>
        </div>
      </div>

      <!-- 统计数据面板 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-green-500/20 rounded-xl p-6 mb-6 shadow-2xl hover:shadow-green-500/10 transition-all duration-300">
        <div class="mb-4">
          <div class="flex items-center gap-3 relative">
            <div class="text-xl animate-icon-glow">📈</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">监控统计</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-green-500/60 via-cyan-500/60 to-transparent ml-4 relative overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-line-scan"></div>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <!-- 上次检查时间 -->
          <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/20 rounded-lg p-4 relative overflow-hidden group">
            <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-blue-500/80 to-transparent"></div>
            <div class="flex items-center gap-4">
              <div class="text-3xl animate-icon-glow">⏰</div>
              <div class="flex-1">
                <div class="text-sm font-medium text-slate-400 mb-1">上次检查时间</div>
                <div class="text-lg font-bold text-blue-400 font-mono">{{ lastCheckTime }}</div>
              </div>
            </div>
          </div>

          <!-- 数据合格率 -->
          <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-green-500/20 rounded-lg p-4 relative overflow-hidden group">
            <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-green-500/80 to-transparent"></div>
            <div class="flex items-center gap-4">
              <div class="text-3xl animate-icon-glow">✅</div>
              <div class="flex-1">
                <div class="text-sm font-medium text-slate-400 mb-1">数据合格率</div>
                <div class="text-lg font-bold font-mono" :class="qualificationRate >= 90 ? 'text-green-400' : qualificationRate >= 70 ? 'text-yellow-400' : 'text-red-400'">
                  {{ qualificationRate }}%
                </div>
              </div>
            </div>
          </div>

          <!-- 数据条数 -->
          <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-purple-500/20 rounded-lg p-4 relative overflow-hidden group">
            <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-purple-500/80 to-transparent"></div>
            <div class="flex items-center gap-4">
              <div class="text-3xl animate-icon-glow">📊</div>
              <div class="flex-1">
                <div class="text-sm font-medium text-slate-400 mb-1">数据条数</div>
                <div class="text-lg font-bold text-purple-400 font-mono">{{ totalDataCount.toLocaleString() }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 数据异常列表 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-green-500/20 rounded-xl shadow-2xl hover:shadow-green-500/10 transition-all duration-300 relative overflow-hidden">
        <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-green-500/80 to-transparent animate-packet-scan"></div>

        <div class="p-6 border-b border-green-500/20">
                      <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="text-2xl animate-icon-glow">🚨</div>
                <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">数据异常详情</h2>

                <!-- 批量操作按钮 -->
                <div v-if="selectedItems.length > 0" class="flex items-center gap-2 ml-4">
                  <button
                    @click="batchAssociateFiles"
                    :disabled="!dataDirectory || isAssociating"
                    class="px-3 py-1.5 bg-gradient-to-r from-green-500/10 to-emerald-500/10 border border-green-500/30 rounded-lg text-green-300 hover:from-green-500/20 hover:to-emerald-500/20 hover:border-green-500/50 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed text-xs font-medium"
                    title="批量智能关联"
                  >
                    批量关联
                  </button>
                  <button
                    @click="clearSelection"
                    class="px-3 py-1.5 bg-gradient-to-r from-slate-500/10 to-gray-500/10 border border-slate-500/30 rounded-lg text-slate-300 hover:from-slate-500/20 hover:to-gray-500/20 hover:border-slate-500/50 transition-all duration-300 text-xs font-medium"
                    title="清除选择"
                  >
                    清除选择
                  </button>
                </div>
              <div class="flex items-center gap-4">
                <!-- 异常统计 -->
                <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-red-500/20 to-pink-500/20 border border-red-500/30 rounded-full">
                  <div class="w-2 h-2 bg-red-400 rounded-full animate-pulse-custom"></div>
                  <span class="text-sm font-semibold text-red-400">异常: {{ dataList.length }}</span>
                </div>
                <!-- 已关联统计 -->
                <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-green-500/20 to-emerald-500/20 border border-green-500/30 rounded-full">
                  <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse-custom"></div>
                  <span class="text-sm font-semibold text-green-400">已关联: {{ associatedCount }}</span>
                </div>
                <!-- 选中统计 -->
                <div v-if="selectedItems.length > 0" class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-blue-500/20 to-cyan-500/20 border border-blue-500/30 rounded-full">
                  <div class="w-2 h-2 bg-blue-400 rounded-full animate-pulse-custom"></div>
                  <span class="text-sm font-semibold text-blue-400">已选: {{ selectedItems.length }}</span>
                </div>
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
          <div v-else-if="dataList.length === 0" class="p-8 text-center">
            <div class="text-6xl mb-4 opacity-60">📈</div>
            <div class="text-slate-400 text-lg">暂无数据异常记录</div>
            <div class="text-slate-500 text-sm mt-2">系统运行正常，所有数据指标良好</div>
          </div>

          <!-- 数据表格 -->
          <div v-else class="overflow-x-auto">
            <table class="w-full table-fixed">
              <!-- 表头 -->
              <thead class="bg-gradient-to-r from-slate-800/80 to-slate-700/60 border-b border-slate-600/50">
                <tr>
                  <th class="w-12 px-6 py-4 text-center text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    <input
                      type="checkbox"
                      :checked="isAllSelected"
                      @change="toggleSelectAll"
                      class="w-4 h-4 text-green-500 bg-slate-700 border-slate-600 rounded focus:ring-green-500 focus:ring-2"
                    />
                  </th>
                  <th class="w-1/6 px-6 py-4 text-left text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    数据源提供单位
                  </th>
                  <th class="w-1/6 px-6 py-4 text-left text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    数据资源名称
                  </th>
                  <th class="w-1/6 px-6 py-4 text-left text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    数据源问题
                  </th>
                  <th class="w-2/6 px-6 py-4 text-left text-xs font-medium text-slate-300 uppercase tracking-wider border-r border-slate-600/30">
                    关联文件
                  </th>
                  <th class="w-48 px-6 py-4 text-center text-xs font-medium text-slate-300 uppercase tracking-wider">
                    操作
                  </th>
                </tr>
              </thead>

              <!-- 表体 -->
              <tbody class="divide-y divide-slate-700/50">
                <tr
                  v-for="(item, index) in dataList"
                  :key="index"
                  class="hover:bg-slate-800/30 transition-all duration-300 relative group"
                >

                  <!-- 复选框 -->
                  <td class="w-12 px-6 py-4 text-center border-r border-slate-700/30">
                    <input
                      type="checkbox"
                      :checked="selectedItems.includes(index)"
                      @change="toggleSelectItem(index)"
                      class="w-4 h-4 text-green-500 bg-slate-700 border-slate-600 rounded focus:ring-green-500 focus:ring-2"
                    />
                  </td>

                  <!-- 数据源提供单位 -->
                  <td class="w-1/6 px-6 py-4 border-r border-slate-700/30">
                    <div class="text-sm text-blue-300 font-semibold break-words">{{ item.provider }}</div>
                  </td>

                  <!-- 数据资源名称 -->
                  <td class="w-1/6 px-6 py-4 border-r border-slate-700/30">
                    <div class="text-sm text-cyan-300 font-semibold break-words">{{ item.resourceName }}</div>
                  </td>

                  <!-- 数据源问题 -->
                  <td class="w-1/6 px-6 py-4 border-r border-slate-700/30">
                    <div class="text-sm text-red-300 font-semibold break-words">{{ item.issue }}</div>
                  </td>

                  <!-- 关联文件 -->
                  <td class="w-2/6 px-6 py-4 border-r border-slate-700/30">
                    <AssociatedFileCard
                      :associated-file="item.associatedFile"
                      :data-directory="dataDirectory"
                      :is-searching-files="isSearchingFiles"
                      @open-file="openFile"
                      @reassociate="manualAssociateFile(item, index)"
                      @remove-association="removeAssociation(item, index)"
                      @smart-associate="findRelatedFiles(item, index)"
                      @manual-associate="manualAssociateFile(item, index)"
                    />
                  </td>

                  <!-- 操作按钮 -->
                  <td class="w-48 px-6 py-4 text-center">
                    <div class="flex flex-col gap-2">
                      <!-- 删除数据源 -->
                      <button
                        @click="deleteDataSource(item, index)"
                        class="px-3 py-1.5 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded text-red-300 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 transition-all duration-300 text-xs font-medium"
                        title="删除数据源"
                      >
                        删除数据源
                      </button>

                      <!-- 修改为手动更新 -->
                      <button
                        @click="setManualUpdate(item, index)"
                        class="px-3 py-1.5 bg-gradient-to-r from-yellow-500/10 to-orange-500/10 border border-yellow-500/30 rounded text-yellow-300 hover:from-yellow-500/20 hover:to-orange-500/20 hover:border-yellow-500/50 transition-all duration-300 text-xs font-medium"
                        title="修改为手动更新"
                      >
                        手动更新
                      </button>

                      <!-- 上传文件更新 -->
                      <button
                        @click="uploadFileUpdate(item, index)"
                        class="px-3 py-1.5 bg-gradient-to-r from-purple-500/10 to-indigo-500/10 border border-purple-500/30 rounded text-purple-300 hover:from-purple-500/20 hover:to-indigo-500/20 hover:border-purple-500/50 transition-all duration-300 text-xs font-medium"
                        title="上传文件更新"
                      >
                        上传更新
                      </button>
                    </div>
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
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { open as shellOpen } from '@tauri-apps/plugin-shell'
import { useBiPlatformStore } from '../../stores/biPlatformStore.ts'
import AssociatedFileCard from './components/AssociatedFileCard.vue'

const router = useRouter()
const biPlatformStore = useBiPlatformStore()

// 响应式数据
const lastCheckTime = ref('2024-01-15 14:30:25')
const qualificationRate = ref(87.5)
const totalDataCount = ref(125430)
const isLoading = ref(false)
const isSearchingFiles = ref(false)
const isAssociating = ref(false)
const dataDirectory = ref('')
const error = ref('')

// 多选相关数据
const selectedItems = ref<number[]>([])

// 监控状态
const isMonitoring = computed(() => !!dataDirectory.value)

// 已关联数量
const associatedCount = computed(() => {
  return dataList.value.filter(item => item.associatedFile).length
})

// 全选状态
const isAllSelected = computed(() => {
  return dataList.value.length > 0 && selectedItems.value.length === dataList.value.length
})

// 数据列表接口
interface DataItem {
  provider: string
  resourceName: string
  issue: string
  associatedFile?: AssociatedFile  // 改为单个文件关联
}

interface AssociatedFile {
  path: string
  similarity: number
}

// 模拟数据
const dataList = ref<DataItem[]>([
  {
    provider: '重庆市教育委员会',
    resourceName: '学校基础信息数据',
    issue: '数据格式不符合标准，缺少必填字段'
  },
  {
    provider: '重庆市卫生健康委员会',
    resourceName: '医疗机构统计数据',
    issue: '数据更新不及时，存在过期记录'
  },
  {
    provider: '重庆市交通局',
    resourceName: '公共交通运营数据',
    issue: '数据重复率过高，影响统计准确性'
  },
  {
    provider: '重庆市公安局',
    resourceName: '人口户籍管理数据',
    issue: '数据加密级别不足，存在安全隐患'
  },
  {
    provider: '重庆市财政局',
    resourceName: '财政收支统计数据',
    issue: '数据分类不准确，影响财务分析'
  },
  {
    provider: '重庆市人力资源和社会保障局',
    resourceName: '就业人员信息数据',
    issue: '数据完整性不足，缺失关键信息'
  },
  {
    provider: '重庆市生态环境局',
    resourceName: '环境监测数据',
    issue: '数据采集频率过低，无法及时反映环境变化'
  },
  {
    provider: '重庆市文化和旅游发展委员会',
    resourceName: '旅游景点信息数据',
    issue: '数据更新滞后，景点信息不准确'
  },
  {
    provider: '重庆市市场监督管理局',
    resourceName: '企业注册登记数据',
    issue: '数据重复录入，造成统计偏差'
  },
  {
    provider: '重庆市发展和改革委员会',
    resourceName: '经济发展指标数据',
    issue: '数据口径不统一，影响政策制定'
  },
  {
    provider: '重庆市规划和自然资源局',
    resourceName: '土地利用规划数据',
    issue: '数据精度不足，影响规划决策'
  },
  {
    provider: '重庆市住房和城乡建设委员会',
    resourceName: '建筑工程项目数据',
    issue: '数据验证机制缺失，存在虚假信息'
  },
  {
    provider: '重庆市水利局',
    resourceName: '水资源监测数据',
    issue: '数据采集设备故障，影响监测准确性'
  },
  {
    provider: '重庆市农业农村委员会',
    resourceName: '农业生产统计数据',
    issue: '数据上报不及时，影响农业政策制定'
  },
  {
    provider: '重庆市商务委员会',
    resourceName: '对外贸易数据',
    issue: '数据分类标准不统一，影响贸易分析'
  },
  {
    provider: '重庆市科学技术局',
    resourceName: '科技创新项目数据',
    issue: '数据质量参差不齐，影响项目评估'
  },
  {
    provider: '重庆市民政局',
    resourceName: '社会组织登记数据',
    issue: '数据更新频率过低，信息滞后严重'
  },
  {
    provider: '重庆市司法局',
    resourceName: '法律援助服务数据',
    issue: '数据统计口径不一致，影响服务质量评估'
  },
  {
    provider: '重庆市审计局',
    resourceName: '审计项目数据',
    issue: '数据保密级别设置不当，存在泄露风险'
  },
  {
    provider: '重庆市体育局',
    resourceName: '体育设施使用数据',
    issue: '数据采集方式落后，影响设施管理效率'
  }
])

// 返回控制台
const goBack = () => {
  router.push('/')
}

// 选择数据目录
const selectDataDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择Excel数据文件存储目录'
    })

    if (selected) {
      dataDirectory.value = selected as string
      // 保存配置到本地存储
      localStorage.setItem('biDataDirectory', dataDirectory.value)
      error.value = ''
      console.log('✅ 数据目录配置成功:', dataDirectory.value)
    }
  } catch (err) {
    console.error('❌ 选择数据目录失败:', err)
    error.value = '选择数据目录失败，请重试'
  }
}

// 一键关联所有文件
const associateAllFiles = async () => {
  if (!dataDirectory.value || dataList.value.length === 0) return

  isAssociating.value = true
  error.value = ''

  try {
    console.log('🔗 开始一键关联所有文件...')

    for (let i = 0; i < dataList.value.length; i++) {
      const item = dataList.value[i]
      if (!item.associatedFile) {
        await findRelatedFiles(item, i, false) // 静默模式，不更新单个搜索状态
      }
    }

    console.log('✅ 一键关联完成')
  } catch (err) {
    console.error('❌ 一键关联失败:', err)
    error.value = '一键关联失败，请检查数据目录配置'
  } finally {
    isAssociating.value = false
  }
}



// 刷新数据
const refreshData = async () => {
  isLoading.value = true
  error.value = ''

  try {
    // 模拟API调用
    await new Promise(resolve => setTimeout(resolve, 1500))

    // 更新统计数据
    lastCheckTime.value = new Date().toLocaleString('zh-CN')
    qualificationRate.value = Math.round((Math.random() * 20 + 80) * 10) / 10
    totalDataCount.value = Math.floor(Math.random() * 50000 + 100000)

    console.log('✅ 数据刷新完成')
  } catch (err) {
    console.error('❌ 刷新数据失败:', err)
    error.value = '刷新数据失败，请检查网络连接'
  } finally {
    isLoading.value = false
  }
}

// 发送BI平台查询
const sendBiQuery = async () => {
  try {
    console.log('开始发送BI平台查询...')
    const response = await biPlatformStore.sendBiQuery()
    console.log('BI平台查询成功:', response)

    // 可以在这里处理响应数据
    if (response.success && response.data) {
      console.log('查询数据:', response.data)
      // 更新统计数据
      lastCheckTime.value = new Date().toLocaleString('zh-CN')
      qualificationRate.value = Math.round((Math.random() * 20 + 80) * 10) / 10
      totalDataCount.value = Math.floor(Math.random() * 50000 + 100000)
    }
  } catch (error) {
    console.error('BI平台查询失败:', error)
  }
}

// 查找关联文件
const findRelatedFiles = async (item: DataItem, index: number, updateSearchingState = true) => {
  if (!dataDirectory.value) {
    error.value = '请先配置数据目录'
    return
  }

  if (updateSearchingState) {
    isSearchingFiles.value = true
  }

  try {
    // 调用Rust后端的文件匹配服务
    const searchText = `${item.provider} ${item.resourceName}`
    const relatedFiles = await invoke('find_similar_files', {
      folderPath: dataDirectory.value,
      searchText: searchText,
      maxResults: 1  // 只需要最相似的一个文件
    }) as AssociatedFile[]

    // 如果找到文件，关联最相似的一个
    if (relatedFiles.length > 0) {
      dataList.value[index].associatedFile = relatedFiles[0]
      console.log(`✅ 自动关联文件: ${relatedFiles[0].path}`)
    } else {
      console.log(`ℹ️ 未找到相关文件`)
    }
  } catch (err) {
    console.error('❌ 查找文件失败:', err)
    if (updateSearchingState) {
      error.value = '查找文件失败，请检查数据目录配置'
    }
  } finally {
    if (updateSearchingState) {
      isSearchingFiles.value = false
    }
  }
}

// 手动关联文件
const manualAssociateFile = async (item: DataItem, index: number) => {
  try {
    const selected = await open({
      multiple: false,
      title: '选择要关联的文件',
      filters: [
        {
          name: 'Excel文件',
          extensions: ['xlsx', 'xls', 'csv']
        },
        {
          name: '所有文件',
          extensions: ['*']
        }
      ]
    })

    if (selected) {
      // 手动选择的文件相似度设为1.0
      dataList.value[index].associatedFile = {
        path: selected as string,
        similarity: 1.0
      }
      error.value = ''
      console.log(`✅ 手动关联文件: ${selected}`)
    }
  } catch (err) {
    console.error('❌ 手动关联文件失败:', err)
    error.value = '选择文件失败，请重试'
  }
}

// 取消关联
const removeAssociation = (item: DataItem, index: number) => {
  dataList.value[index].associatedFile = undefined
  console.log(`🗑️ 取消关联文件`)
}

// 打开文件
const openFile = async (filePath: string) => {
  try {
    await shellOpen(filePath)
    console.log(`📂 打开文件: ${filePath}`)
  } catch (err) {
    console.error('❌ 打开文件失败:', err)
    error.value = '打开文件失败，请检查文件是否存在'
  }
}

// 切换全选状态
const toggleSelectAll = () => {
  if (isAllSelected.value) {
    selectedItems.value = []
  } else {
    selectedItems.value = dataList.value.map((_, index) => index)
  }
}

// 切换单个项目选择状态
const toggleSelectItem = (index: number) => {
  const itemIndex = selectedItems.value.indexOf(index)
  if (itemIndex > -1) {
    selectedItems.value.splice(itemIndex, 1)
  } else {
    selectedItems.value.push(index)
  }
}

// 批量关联文件
const batchAssociateFiles = async () => {
  if (!dataDirectory.value || selectedItems.value.length === 0) return

  isAssociating.value = true
  error.value = ''

  try {
    console.log('🔗 开始批量关联文件...')

    for (const index of selectedItems.value) {
      const item = dataList.value[index]
      if (!item.associatedFile) {
        await findRelatedFiles(item, index, false) // 静默模式
      }
    }

    console.log('✅ 批量关联完成')
  } catch (err) {
    console.error('❌ 批量关联失败:', err)
    error.value = '批量关联失败，请检查数据目录配置'
  } finally {
    isAssociating.value = false
  }
}

// 清除选择
const clearSelection = () => {
  selectedItems.value = []
}

// 删除数据源
const deleteDataSource = (item: DataItem, index: number) => {
  if (confirm(`确定要删除数据源"${item.resourceName}"吗？此操作不可恢复。`)) {
    dataList.value.splice(index, 1)
    console.log(`🗑️ 删除数据源: ${item.resourceName}`)
  }
}

// 修改为手动更新
const setManualUpdate = (item: DataItem, index: number) => {
  // 这里可以添加修改更新模式的逻辑
  console.log(`🔄 修改数据源为手动更新: ${item.resourceName}`)
  // 可以显示一个提示或更新数据源的状态
}

// 上传文件更新
const uploadFileUpdate = async (item: DataItem, index: number) => {
  try {
    const selected = await open({
      multiple: false,
      title: '选择要上传的更新文件',
      filters: [
        {
          name: 'Excel文件',
          extensions: ['xlsx', 'xls', 'csv']
        },
        {
          name: '所有文件',
          extensions: ['*']
        }
      ]
    })

    if (selected) {
      console.log(`📤 上传文件更新: ${selected}`)
      // 这里可以添加文件上传和处理的逻辑
    }
  } catch (err) {
    console.error('❌ 上传文件失败:', err)
    error.value = '上传文件失败，请重试'
  }
}

// 组件挂载时加载配置
onMounted(() => {
  // 恢复上次配置的数据目录
  const savedDirectory = localStorage.getItem('biDataDirectory')
  if (savedDirectory) {
    dataDirectory.value = savedDirectory
  }
})
</script>

<style scoped>
@import '../../views/ProxyController/animations.css';

/* 自定义动画 */
@keyframes header-scan {
  0% { transform: translateX(-100%); opacity: 0; }
  50% { opacity: 1; }
  100% { transform: translateX(100%); opacity: 0; }
}

@keyframes packet-scan {
  0% { transform: translateX(-100%); opacity: 0; }
  50% { opacity: 1; }
  100% { transform: translateX(100%); opacity: 0; }
}

@keyframes line-scan {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

@keyframes pulse-custom {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes status-blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0.3; }
}

@keyframes icon-glow {
  0%, 100% { filter: drop-shadow(0 0 5px rgba(34, 197, 94, 0.5)); }
  50% { filter: drop-shadow(0 0 15px rgba(34, 197, 94, 0.8)); }
}

@keyframes icon-pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.animate-header-scan { animation: header-scan 3s ease-in-out infinite; }
.animate-packet-scan { animation: packet-scan 4s ease-in-out infinite; }
.animate-line-scan { animation: line-scan 2s ease-in-out infinite; }
.animate-pulse-custom { animation: pulse-custom 2s ease-in-out infinite; }
.animate-status-blink { animation: status-blink 1s ease-in-out infinite; }
.animate-icon-glow { animation: icon-glow 3s ease-in-out infinite; }
.animate-icon-pulse { animation: icon-pulse 2s ease-in-out infinite; }

.animation-delay-1_5s { animation-delay: 1.5s; }
.animation-delay-2s { animation-delay: 2s; }
</style>

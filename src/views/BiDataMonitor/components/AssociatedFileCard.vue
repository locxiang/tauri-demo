<template>
  <div class="space-y-2">
    <div v-if="associatedFile" class="p-3 bg-gradient-to-r from-green-500/10 to-emerald-500/10 border border-green-500/20 rounded-lg">
      <!-- 文件信息行 -->
      <div class="flex-1 min-w-0">
        <!-- 文件名 -->
        <div class="text-sm text-green-300 font-semibold break-all">
          📄 {{ getFileName(associatedFile.path) }}
        </div>
        <!-- 相对路径 -->
        <div class="text-xs text-slate-400 font-mono break-all mt-1" :title="associatedFile.path">
          {{ getRelativePath(associatedFile.path) }}
        </div>
        <!-- 相似度进度条 -->
        <div class="flex items-center gap-2 mt-2">
          <span class="text-xs text-slate-500">相似度:</span>
          <div class="flex-1 max-w-24 bg-slate-700 rounded-full h-1.5 relative overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-green-500 to-cyan-500 rounded-full transition-all duration-500"
              :style="{ width: `${associatedFile.similarity * 100}%` }"
            ></div>
          </div>
          <span class="text-xs font-semibold text-green-400">{{ (associatedFile.similarity * 100).toFixed(1) }}%</span>
        </div>
      </div>
      <!-- 操作按钮行 -->
      <div class="flex items-center gap-2 mt-3 pt-3 border-t border-green-500/20">
        <button
          @click="$emit('openFile', associatedFile.path)"
          class="px-3 py-1.5 bg-gradient-to-r from-blue-500/10 to-cyan-500/10 border border-blue-500/30 rounded text-blue-300 hover:from-blue-500/20 hover:to-cyan-500/20 hover:border-blue-500/50 transition-all duration-300 text-xs font-medium"
          title="打开文件"
        >
          打开文件
        </button>
        <button
          @click="$emit('reassociate')"
          class="px-3 py-1.5 bg-gradient-to-r from-blue-500/10 to-indigo-500/10 border border-blue-500/30 rounded text-blue-300 hover:from-blue-500/20 hover:to-indigo-500/20 hover:border-blue-500/50 transition-all duration-300 text-xs font-medium"
          title="重新选择文件"
        >
          重新选择
        </button>
        <button
          @click="$emit('removeAssociation')"
          class="px-3 py-1.5 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded text-red-300 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 transition-all duration-300 text-xs font-medium"
          title="取消关联"
        >
          取消关联
        </button>
      </div>
    </div>
    <div v-else class="space-y-2">
      <div class="text-slate-500 text-sm py-2">未关联文件</div>
      <!-- 操作按钮行 -->
      <div class="flex items-center gap-2">
        <!-- 智能关联按钮 -->
        <button
          @click="$emit('smartAssociate')"
          :disabled="!dataDirectory || isSearchingFiles"
          class="px-3 py-1.5 bg-gradient-to-r from-green-500/10 to-emerald-500/10 border border-green-500/30 rounded-lg text-green-300 hover:from-green-500/20 hover:to-emerald-500/20 hover:border-green-500/50 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed text-xs font-medium"
          title="智能搜索关联文件"
        >
          {{ isSearchingFiles ? '搜索中...' : '智能关联' }}
        </button>
        <!-- 手动选择按钮 -->
        <button
          @click="$emit('manualAssociate')"
          class="px-3 py-1.5 bg-gradient-to-r from-blue-500/10 to-indigo-500/10 border border-blue-500/30 rounded-lg text-blue-300 hover:from-blue-500/20 hover:to-indigo-500/20 hover:border-blue-500/50 transition-all duration-300 text-xs font-medium"
          title="手动选择文件"
        >
          手动选择
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface AssociatedFile {
  path: string
  similarity: number
}

interface Props {
  associatedFile?: AssociatedFile
  dataDirectory?: string
  isSearchingFiles?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  dataDirectory: '',
  isSearchingFiles: false
})

const emit = defineEmits<{
  openFile: [filePath: string]
  reassociate: []
  removeAssociation: []
  smartAssociate: []
  manualAssociate: []
}>()

// 获取文件名（不包含路径）
const getFileName = (filePath: string): string => {
  return filePath.split(/[/\\]/).pop() || filePath
}

// 获取相对路径（显示目录结构）
const getRelativePath = (filePath: string): string => {
  const pathParts = filePath.split(/[/\\]/)
  // 如果路径太长，只显示最后几级目录
  if (pathParts.length > 3) {
    return '.../' + pathParts.slice(-4, -1).join('/') + '/'
  } else {
    return pathParts.slice(0, -1).join('/') + '/'
  }
}
</script>

<template>
  <div class="network-device-container">
    <div class="card">
      <h2>网络设备信息</h2>

      <div class="button-group">
        <button @click="getDeviceCount" :disabled="loading">
          获取网卡数量
        </button>
        <button @click="getActiveDeviceCount" :disabled="loading">
          获取活跃网卡数量
        </button>
        <button @click="getAllDevices" :disabled="loading">
          获取所有网络设备
        </button>
      </div>

      <div v-if="loading" class="loading">
        正在加载...
      </div>

      <div v-if="deviceCount !== null" class="result">
        <h3>网卡总数: {{ deviceCount }}</h3>
      </div>

      <div v-if="activeDeviceCount !== null" class="result">
        <h3>活跃网卡数量: {{ activeDeviceCount }}</h3>
      </div>

      <div v-if="devices.length > 0" class="devices-list">
        <h3>网络设备列表:</h3>
        <div class="device-grid">
          <div
            v-for="(device, index) in devices"
            :key="index"
            class="device-card"
            :class="{
              'active': device.is_up && device.is_running,
              'loopback': device.is_loopback
            }"
          >
            <div class="device-name">{{ device.name }}</div>
            <div class="device-description">
              {{ device.description || '无描述' }}
            </div>
            <div class="device-status">
              <span v-if="device.is_up" class="status-badge up">UP</span>
              <span v-if="device.is_running" class="status-badge running">运行中</span>
              <span v-if="device.is_loopback" class="status-badge loopback">回环</span>
            </div>
          </div>
        </div>
      </div>

      <div v-if="error" class="error">
        错误: {{ error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface NetworkDevice {
  name: string
  description: string | null
  is_up: boolean
  is_running: boolean
  is_loopback: boolean
}

const loading = ref(false)
const deviceCount = ref<number | null>(null)
const activeDeviceCount = ref<number | null>(null)
const devices = ref<NetworkDevice[]>([])
const error = ref('')

const clearResults = () => {
  deviceCount.value = null
  activeDeviceCount.value = null
  devices.value = []
  error.value = ''
}

const getDeviceCount = async () => {
  loading.value = true
  clearResults()

  try {
    const count = await invoke<number>('get_network_device_count')
    deviceCount.value = count
  } catch (err) {
    error.value = String(err)
  } finally {
    loading.value = false
  }
}

const getActiveDeviceCount = async () => {
  loading.value = true
  clearResults()

  try {
    const count = await invoke<number>('get_active_network_device_count')
    activeDeviceCount.value = count
  } catch (err) {
    error.value = String(err)
  } finally {
    loading.value = false
  }
}

const getAllDevices = async () => {
  loading.value = true
  clearResults()

  try {
    const deviceList = await invoke<NetworkDevice[]>('get_network_devices')
    devices.value = deviceList
  } catch (err) {
    error.value = String(err)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.network-device-container {
  max-width: 800px;
  margin: 20px auto;
  padding: 20px;
}

.card {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e5e7eb;
}

h2 {
  color: #1f2937;
  margin-bottom: 20px;
  font-size: 24px;
  font-weight: 600;
}

.button-group {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

button {
  background: #3b82f6;
  color: white;
  border: none;
  padding: 10px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background-color 0.2s;
}

button:hover:not(:disabled) {
  background: #2563eb;
}

button:disabled {
  background: #9ca3af;
  cursor: not-allowed;
}

.loading {
  text-align: center;
  color: #6b7280;
  font-style: italic;
  margin: 20px 0;
}

.result {
  background: #f0f9ff;
  border: 1px solid #0ea5e9;
  border-radius: 6px;
  padding: 16px;
  margin: 16px 0;
}

.result h3 {
  color: #0c4a6e;
  margin: 0;
  font-size: 18px;
}

.devices-list {
  margin-top: 20px;
}

.devices-list h3 {
  color: #1f2937;
  margin-bottom: 16px;
  font-size: 18px;
}

.device-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.device-card {
  background: #f9fafb;
  border: 2px solid #e5e7eb;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s;
}

.device-card.active {
  border-color: #10b981;
  background: #ecfdf5;
}

.device-card.loopback {
  border-color: #f59e0b;
  background: #fffbeb;
}

.device-name {
  font-weight: 600;
  font-size: 16px;
  color: #1f2937;
  margin-bottom: 8px;
}

.device-description {
  color: #6b7280;
  font-size: 14px;
  margin-bottom: 12px;
  min-height: 20px;
}

.device-status {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.status-badge {
  font-size: 12px;
  font-weight: 500;
  padding: 4px 8px;
  border-radius: 4px;
  text-transform: uppercase;
}

.status-badge.up {
  background: #dcfce7;
  color: #166534;
}

.status-badge.running {
  background: #dbeafe;
  color: #1e40af;
}

.status-badge.loopback {
  background: #fef3c7;
  color: #92400e;
}

.error {
  background: #fef2f2;
  border: 1px solid #fca5a5;
  color: #dc2626;
  padding: 16px;
  border-radius: 6px;
  margin: 16px 0;
  word-break: break-word;
}
</style>

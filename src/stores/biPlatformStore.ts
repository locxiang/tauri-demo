import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// BI查询请求接口
export interface BiQueryRequest {
  olap_query_param: string;
  component_id: string;
  component_type: string;
  report_id: string;
  last_refresh_time: number;
}

// BI查询响应接口
export interface BiQueryResponse {
  success: boolean;
  data?: any;
  message?: string;
  error_code?: string;
}

export const useBiPlatformStore = defineStore('biPlatform', () => {
  // 状态
  const isLoading = ref<boolean>(false);
  const error = ref<string>('');
  const lastResponse = ref<BiQueryResponse | null>(null);
  const queryHistory = ref<BiQueryRequest[]>([]);

  // 默认查询参数（基于你提供的curl命令）
  const defaultQueryParams = {
    olap_query_param: `%7B%22componentId%22%3A%22gs5yyp7s%22%2C%22componentName%22%3A%22Sheet1%21A1%22%2C%22configs%22%3A%5B%7B%22type%22%3A%22field%22%2C%22config%22%3A%7B%22fields%22%3A%5B%7B%22guid%22%3A%22cf1b3b67-1211-4059-a358-b5f40789602c%22%2C%22fid%22%3A%22bbafe16af3%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22a2bb19a4-9a69-4b3e-bfad-e37e15bf14c2%22%2C%22fid%22%3A%2292fa3dd229%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22c2e713ec-d165-4c17-af16-7ad96e5345a1%22%2C%22fid%22%3A%227e954d6c5d%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%2251873768-bd11-4d9d-8273-737b02ce1c0c%22%2C%22fid%22%3A%22e32e23ee19%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22ab8929e7-6b97-475c-a1ac-bfed2e9bfa58%22%2C%22fid%22%3A%22c6da58ef1c%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%2289e6c818-52be-44f1-be61-7e0bba069fe9%22%2C%22fid%22%3A%229d4db0ec2c%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22d683ebd5-e05c-4de6-a8c5-28a810fdf626%22%2C%22fid%22%3A%223ac9584256%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%224957b23a-3cf0-4275-ad88-4db6011ea1d5%22%2C%22fid%22%3A%22bfa140936c%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%221fc65896-5d91-4f06-949f-797b7b1f4106%22%2C%22fid%22%3A%22e783143093%22%2C%22areaType%22%3A%22row%22%7D%5D%7D%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%7D%2C%7B%22type%22%3A%22paging%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22limit%22%3A30000%2C%22offset%22%3A0%2C%22pagedByAllDim%22%3Atrue%7D%7D%2C%7B%22type%22%3A%22queryConfig%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22needCount%22%3Atrue%2C%22queryCount%22%3Atrue%2C%22queryDetail%22%3Atrue%7D%7D%2C%7B%22type%22%3A%22advancedParam%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22autoInsightParam%22%3A%7B%22enable%22%3Afalse%7D%2C%22wordCloudParam%22%3A%7B%7D%2C%22summarizeParams%22%3A%5B%5D%2C%22trendLineParams%22%3A%5B%5D%2C%22forecastParams%22%3A%5B%5D%2C%22anomalyDetectionParams%22%3A%5B%5D%2C%22clusteringParams%22%3A%5B%5D%2C%22groupParam%22%3Anull%7D%7D%2C%7B%22type%22%3A%22annotationParam%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22measureThresholdParams%22%3A%5B%5D%2C%22inflectionPointParams%22%3A%5B%5D%7D%7D%5D%2C%22dataType%22%3A%22general%22%2C%22reportId%22%3A%2274271567-0203-4ff7-b43f-6195e598a8b1%22%7D`,
    component_id: 'gs5yyp7s',
    component_type: '77',
    report_id: '74271567-0203-4ff7-b43f-6195e598a8b1',
    last_refresh_time: Date.now() / 1000, // 当前时间戳
  };

  // 计算属性
  const hasError = computed(() => !!error.value);
  const hasResponse = computed(() => !!lastResponse.value);

  // 发送BI平台查询请求
  const sendBiQuery = async (): Promise<BiQueryResponse> => {
    try {
      isLoading.value = true;
      error.value = '';

      console.log('发送BI平台查询请求...');

      // 调用后端API，不需要传递任何参数
      const response: BiQueryResponse = await invoke('send_bi_query');

      console.log('BI平台查询响应:', response);

      // 保存响应和查询历史
      lastResponse.value = response;
      queryHistory.value.push({
        olap_query_param: defaultQueryParams.olap_query_param,
        component_id: defaultQueryParams.component_id,
        component_type: defaultQueryParams.component_type,
        report_id: defaultQueryParams.report_id,
        last_refresh_time: Date.now() / 1000,
      });

      if (!response.success) {
        error.value = response.message || '查询失败';
      }

      return response;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '未知错误';
      error.value = `BI平台查询失败: ${errorMessage}`;
      console.error('BI平台查询错误:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  // 发送默认查询（保持向后兼容）
  const sendDefaultQuery = async (): Promise<BiQueryResponse> => {
    return sendBiQuery();
  };

  // 清除错误
  const clearError = () => {
    error.value = '';
  };

  // 清除响应
  const clearResponse = () => {
    lastResponse.value = null;
  };

  // 清除历史记录
  const clearHistory = () => {
    queryHistory.value = [];
  };

  // 获取查询历史
  const getQueryHistory = () => {
    return queryHistory.value;
  };

  return {
    // 状态
    isLoading,
    error,
    lastResponse,
    queryHistory,

    // 计算属性
    hasError,
    hasResponse,

    // 方法
    sendBiQuery,
    sendDefaultQuery,
    clearError,
    clearResponse,
    clearHistory,
    getQueryHistory,
  };
});

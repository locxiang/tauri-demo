import { acceptHMRUpdate, defineStore } from 'pinia'

const versionString =
  import.meta.env.MODE === 'development' ? `${import.meta.env.VITE_APP_VERSION}-dev` : import.meta.env.VITE_APP_VERSION


export interface AppState {
  debug: boolean;
  version: string;
  isInitialized: boolean;
  name: string;
}
console.log("import.meta.env", import.meta.env)

export const useAppStore = defineStore('app', {
  state: () => ({
    debug: import.meta.env.MODE === 'development',
    version: versionString,
    isInitialized: false,
    name: import.meta.env.VITE_APP_NAME,
  }),

  actions: {
    initApp() {
      this.isInitialized = true
      console.log('app initialized!')
    },
  },

  getters: {
    isReady: (state) => {
      return !state.isInitialized
    },

    storeGreet: (state) => {
      if (state.name.length > 0) {
        return `Greetings from Pinia store, ${state.name}!`
      }
      return ''
    },
  },
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot))
}

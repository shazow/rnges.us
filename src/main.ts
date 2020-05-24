import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

async function init() {
  const wasm = await import("wasm-net")

  Vue.config.productionTip = false
  Vue.prototype.$wasm = wasm
  // Vue.prototype.appname = "hello"

  new Vue({
    router,
    store,
    render: h => h(App, { props: { wasm: wasm } })
  }).$mount('#app')
}
init()



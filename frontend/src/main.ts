import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import 'roboto-fontface/css/roboto/roboto-fontface.css'


import router from './router'

Vue.config.productionTip = false

/* eslint-disable-line no-new */
new Vue({
  el: '#app',
  router,
  vuetify,
  render: (h) => h(App),
  components: { App }
})

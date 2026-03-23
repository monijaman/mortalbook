import '@mdi/font/css/materialdesignicons.css'
import { createApp } from 'vue'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import 'vuetify/styles'
import App from './App.vue'
import router from './router'
import store from './store'

const vuetify = createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'light',
        themes: {
            light: {
                colors: {
                    primary: '#5C6BC0',
                    secondary: '#7E57C2',
                    accent: '#26C6DA',
                    error: '#EF5350',
                    warning: '#FFA726',
                    info: '#42A5F5',
                    success: '#66BB6A',
                    surface: '#FFFFFF',
                    background: '#F5F5F7',
                },
            },
        },
    },
    icons: { defaultSet: 'mdi' },
})

const app = createApp(App)

app.use(store)
app.use(router)
app.use(vuetify)

app.mount('#app')

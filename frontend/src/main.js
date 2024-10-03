import { createApp } from 'vue'
import './style.scss'
import App from './App.vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'

let routes = [
    { path: '/', component: () => import('@pages/Home.vue') },
    { path: '/containers', component: () => import('@pages/Containers.vue') },
    { path: '/credits', component: () => import('@pages/Credits.vue') },
    { path: '/:pathMatch(.*)*', component: () => import('@pages/NotFound.vue') },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
})

createApp(App).use(createPinia()).use(router).mount('#app')

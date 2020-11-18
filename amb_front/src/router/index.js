import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import UserPage from '../views/UserPage.vue'
import RegistrationPage from '../views/RegistrationPage'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
  },
  {
    path: '/user',
    name: 'UserPage',
    component: UserPage
  },
  {
    path: '/registration',
    name: 'RegistrationPage',
    component: RegistrationPage
  },
  {
    path: "*",
    name: "PageNotFound",
    component:() => import('../views/PageNotFound.vue')
  }
]

const router = new VueRouter({
  routes
})

export default router

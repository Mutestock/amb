import Vue from 'vue'
import VueRouter from 'vue-router'
import HomePage from '../views/HomePage.vue'
import UserPage from '../views/UserPage.vue'
import RegistrationPage from '../views/RegistrationPage'
import TrackPage from '../views/TrackPage'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'HomePage',
    component: HomePage
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
    path: '/track',
    name: 'TrackPage',
    component: TrackPage
  },
  {
    path: "*",
    name: "PageNotFound",
    component: () => import('../views/error_pages/PageNotFound.vue')
  }
]

const router = new VueRouter({
  routes
})

export default router

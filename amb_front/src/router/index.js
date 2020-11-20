import Vue from 'vue'
import VueRouter from 'vue-router'
import HomePage from '../views/HomePage.vue'
import UserPage from '../views/UserPage.vue'
import RegistrationPage from '../views/RegistrationPage'
import TrackPage from '../views/TrackPage'
import LoginPage from '../views/LoginPage'

Vue.use(VueRouter)

export const router = new VueRouter({
  mode: 'history',
  routes:[
    { path: '/', component: HomePage },
    { path: '/login', component: LoginPage},
    { path: '/registration', component: RegistrationPage },
    { path: '/user', component: UserPage },
    { path: '/track', component: TrackPage },
    { path: '*', component: () => import('../views/error_pages/PageNotFound.vue')}
  ]
})

router.beforeEach((to, from, next) => {
  const publicPages = ['/login', '/registration'];
  const authRequired = !publicPages.includes(to.path);
  const loggedIn = localStorage.getItem('user');

  if (authRequired && !loggedIn){
    return next('/login')
  }
  next();
})


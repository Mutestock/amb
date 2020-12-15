
import Vue from 'vue'
import VueRouter from 'vue-router'
import HomePage from '../views/HomePage.vue'
import UserPage from '../views/UserPage.vue'
import RegistrationPage from '../views/RegistrationPage'
import TrackPage from '../views/TrackPage'
import LoginPage from '../views/LoginPage';

Vue.use(VueRouter)

const routes = [
  { path: '/', name: 'HomePage', component: HomePage },
  { path: '/about', name: 'About', component: () => import('../views/About.vue') },
  { path: '/user', name: 'UserPage', component: UserPage },
  { path: '/registration', name: 'RegistrationPage', component: RegistrationPage },
  { path: '/track', name: 'TrackPage', component: TrackPage },
  { path: '/login', name: 'LoginPage', component: LoginPage },
  { path: "*", name: "PageNotFound", component: () => import('../views/error_pages/PageNotFound.vue') }
];

const router = new VueRouter({
  mode: "history",
  routes
});

router.beforeEach((to, from, next) => {
  const publicPages = ['/login', '/registration', '/', '/about'];
  const authRequired = !publicPages.includes(to.path);
  const loggedIn = localStorage.getItem('user');
  if (authRequired && !loggedIn) {
    alert("Restricted path. Please log in.")
    return next('/login');
  }
  next();
});

export default router
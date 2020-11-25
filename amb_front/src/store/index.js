import Vuex from 'vuex';
import Vue from 'vue';

import users from './modules/users';
import tracks from './modules/tracks';
import account from './modules/account';
import alert from './modules/alert';

Vue.use(Vuex);

export default new Vuex.Store({
    modules:{
        users,
        tracks,
        alert,
        account
    }
});
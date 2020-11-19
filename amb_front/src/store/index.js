import Vuex from 'vuex';
import Vue from 'vue';
import users from './modules/users';
import tracks from './modules/tracks';

Vue.use(Vuex);

export default new Vuex.Store({
    modules:{
        users,
        tracks
    }
});
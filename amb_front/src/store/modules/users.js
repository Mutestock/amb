import axios from "axios";
import { user_service } from '../../_services/user_service';


const state = {
  currentUser: {}
};

const getters = {
  getCurrentUser: state => {
    return state.currentUser;
  },
  hasLoggedInUser: state => {
    return state.currentUser !== null;
  }
};

const actions = {
  async fetchUser() {
    const response = await axios.get(
      process.env.VUE_APP_BACK_END_HOST + "/api/user/1"
    );
    console.log(response.data)
  },

  async registerUser({ commit }, User) {
    const response = user_service.register(User)
    if (response.data) {
      localStorage.setItem('user', JSON.stringify(User));
      commit("registrationAction", response.data);
    }
    else {
      console.log("No response from server");
    }
  },

  async loginUser({ commit }, { username, password }) {
    const response = user_service.login(username, password);
    if (response.data) {
      localStorage.setItem('user', JSON.stringify(response.data));
      commit("loginAction", response.data);
    }
    else {
      console.log("No response from server");
    }

  }
};

const mutations = {
  loginAction: (state, currentUser) => (state.currentUser = currentUser),
  registrationAction: (state, currentUser) => (state.currentUser = currentUser),
  loginRequest(state, user) {
    state.currentUser = user;
  }
};

export default {
  state,
  getters,
  actions,
  mutations,
};

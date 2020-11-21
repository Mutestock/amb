import axios from "axios";

const state = {
  users: [],
  currentUser: {},
};

const getters = {
  getCurrentUser: state => {
    return state.currentUser;
  },
  hasLoggedInUser: state => {
    return state.currentUser!==null;
  }
};

const actions = {
  async fetchUser() {
    const response = await axios.get(
      process.env.VUE_APP_BACK_END_HOST + "/api/user/1"
    );
    console.log(response.data)
  },
  async registerUser({ commit }, User){
    const response = await axios.post(
      process.env.VUE_APP_BACK_END_HOST + "/api/user", User
    );
    commit("registrationAction", response.data);
    // Need status code from backend. 201
    if(User){
      localStorage.setItem('user', User);
    }
  }
};

const mutations = {
  registrationAction: (state, currentUser) => (state.currentUser = currentUser),
  loginRequest(state, user){
    state.currentUser = user;
  }
};

export default {
  state,
  getters,
  actions,
  mutations,
};

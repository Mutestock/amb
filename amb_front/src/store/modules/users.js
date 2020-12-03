import { userService } from "../../_services/user_service";

const state = {
  currentUser: localStorage.getItem("user")
};

const getters = {
  getCurrentUser: (state) => state.currentUser,
};

const actions = {
  async registerUser({ commit }, User) {
    const response = userService.register(User);
    if (response.data) {
      localStorage.setItem("user", JSON.stringify(User));
      commit("registrationAction", response.data);
    } else {
      console.log("No response from server");
    }
  },

  async loginUser({ commit }, info) {
    userService.login(info.username, info.password).then((data) => {
      if (data) {
        localStorage.setItem("user", JSON.stringify(data.data.token));
        const parseJwt = (token) => {
          try {
            return JSON.parse(atob(token.split(".")[1]));
          } catch (e) {
            return null;
          }
        };
        let parsed = parseJwt(data.data.token);
        commit("loginAction", parsed.user_response);
      } else {
        console.log("No response from server");
      }
    });
  },
};

const mutations = {
  loginAction: (state, currentUser) => (state.currentUser = currentUser),
  registrationAction: (state, currentUser) => (state.currentUser = currentUser),
  LOGIN_CURRENT_USER: (state, currentUser) => (state.currentUser = currentUser),
};

export default {
  state,
  getters,
  actions,
  mutations,
};

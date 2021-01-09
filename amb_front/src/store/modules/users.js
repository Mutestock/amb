import { userService } from "../../_services/user_service";
import router from "../../router";

const parseJwt = (token) => {
  try {
    return JSON.parse(atob(token.split(".")[1]));
  } catch (e) {
    return null;
  }
};

const state = {
  currentUser: localStorage.getItem("user")
    ? parseJwt(localStorage.getItem("user")).user_response
    : {},
  midLoginRedirectionPass: false,
};

const getters = {
  getCurrentUser: (state) => state.currentUser,
  getMidLoginRedirectionPass: (state) => state.midLoginRedirectionPass,
};

const actions = {
  async registerUser({ commit }, User) {
    const response = userService.register(User);
    if (response.data) {
      localStorage.setItem("user", JSON.stringify(User));
      commit("SET_CURRENT_USER", response.data);
    } else {
      console.log("No response from server");
    }
  },

  async loginUser({ commit }, info) {
    userService.login(info.username, info.password).then((data) => {
      if (data) {
        localStorage.setItem("user", JSON.stringify(data.data.token));

        let parsed = parseJwt(data.data.token);
        commit("MID_LOGIN_REDIRECTION_PASS");
        console.log("INSIDE loginUser: " + state.midLoginRedirectionPass);
        commit("SET_CURRENT_USER", parsed.user_response);
        router.push("user");
      } else {
        console.log("No response from server");
      }
    });
  },
  async logoutUser({ commit }) {
    localStorage.removeItem("user");
    commit("LOGOUT_CURRENT_USER");
  },
  async midLoginRedirectionPassFinished({ commit }) {
    commit("MID_LOGIN_REDIRECTION_PASS_FINISHED");
  },
};

const mutations = {
  SET_CURRENT_USER: (state, currentUser) => (state.currentUser = currentUser),
  LOGOUT_CURRENT_USER: (state) => (state.currentUser = ""),
  MID_LOGIN_REDIRECTION_PASS: (state) => (state.midLoginRedirectionPass = true),
  MID_LOGIN_REDIRECTION_PASS_FINISHED: (state) =>
    (state.midLoginRedirectionPass = false),
};

export default {
  state,
  getters,
  actions,
  mutations,
};

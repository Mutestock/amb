import { userService } from "../../_services/user_service";

const state = {
  currentUser: {},
};

const getters = {
  getCurrentUser: state => state.currentUser,
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
        const parseJwt = (token) => {
          try {
            return JSON.parse(atob(token.split(".")[1]));
          } catch (e) {
            return null;
          }
        };
        let parsed = parseJwt(data.data.token);
        console.log(parsed.user_response);
        
        commit("SET_CURRENT_USER", parsed.user_response);
        console.log(state.currentUser);
        
      } else {
        console.log("No response from server");
      }
    });
  },
  async logoutUser({ commit }) {
    localStorage.removeItem("user");
    commit("LOGOUT_CURRENT_USER");
  },
};

const mutations = {
  SET_CURRENT_USER: (state, currentUser) => (state.currentUser = currentUser),
  LOGOUT_CURRENT_USER: (state) =>
    (state.currentUser = "")
};

export default {
  state,
  getters,
  actions,
  mutations,
};

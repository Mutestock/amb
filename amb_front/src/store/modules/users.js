import axios from "axios";

const state = {
  users: [],
};

const getters = {};

const actions = {
  async fetchUser() {
    const response = await axios.get(
      process.env.VUE_APP_BACK_END_HOST+"/api/user/1"
    );
    console.log(response.data);
  },
};

const mutations = {};

export default {
  state,
  getters,
  actions,
  mutations,
};

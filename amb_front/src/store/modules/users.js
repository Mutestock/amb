import axios from "axios";

const state = {
  users: [],
};

const getters = {};

const actions = {
  async fetchUser() {
    const response = await axios.get(
      "$VUE_APP_BACKEND_HOST" + ":8000/api/user/1"
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

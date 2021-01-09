<template>
  <v-app>
    <v-main no-gutters class="login-fields">
      <v-text-field
        label="Username"
        hide-details="auto"
        v-model="username"
      ></v-text-field>
      <v-text-field
        label="Password"
        hide-detials="auto"
        v-model="password"
        type="password"
      ></v-text-field>
      <v-btn label="Login" elevation="4" @click="loginClick">Login</v-btn>
    </v-main>
  </v-app>
</template>

<script>
import { mapActions, } from "vuex";
import store from "../store";
export default {
  data() {
    return {
      username: "",
      password: "",
    };
  },
  methods: {
    ...mapActions(["loginUser"]),
    loginClick() {
      if (!this.emptyFields(this.username, this.password)) {
        let user = {
          username: this.username,
          password: this.password,
        };
        store.dispatch('loginUser',user).then(()=>{
          console.log("boop");
        });
      }
    },
    emptyFields(username, password) {
      if (!username || !password) {
        this.$alert("Please insert information in both fields");
        return true;
      }
      return false;
    },
  },
};
</script>

<style scoped>
.restrict-top {
  overflow: auto;
}
.login-fields {
  align-content: center;
  padding: 50px;
  width: 400px;
  overflow: auto;
  position: absolute;
  top: 0px;
}
</style>
<template>
  <div>
    <div class="login-fields">
      <v-text-field label="Username" hide-details="auto" v-model="username"></v-text-field>
      <v-text-field label="Password" hide-detials="auto" v-model="password" type="password"></v-text-field>
      <v-btn label="Login" elevation="4" @click="loginClick">Login</v-btn>
    </div>
  </div>
</template>

<script>
import { mapActions, mapMutations } from "vuex";
export default {
  data() {
    return {
      username: "",
      password: ""
    };
  },
  methods: {
    ...mapActions(["loginUser"]),
    ...mapMutations(['LOGIN_CURRENT_USER']),
    loginClick() {
      if (!this.emptyFields(this.username, this.password)) {
        console.log(this.username, this.password);
        let user = this.loginUser({"username":this.username, "password":this.password});
        console.log(user);
        this.LOGIN_CURRENT_USER(user);
      }
    },
    emptyFields(username, password) {
      if (!username || !password) {
        this.$alert("Please insert information in both fields");
        return true;
      }
      return false;
    }
  }
};
</script>

<style scoped>
.login-fields {
  align-content: center;
  padding: 50px;
  width: 400px;
}
</style>
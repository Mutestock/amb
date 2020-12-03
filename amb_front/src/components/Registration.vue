<template>
  <div>
    <div class="registration-fields">
      <v-text-field label="Username" hide-details="auto" v-model="username"></v-text-field>
      <v-text-field label="Password" hide-details="auto" v-model="password" type="password"></v-text-field>
      <v-text-field
        label="Retype password"
        hide-details="auto"
        v-model="retypePassword"
        type="password"
      ></v-text-field>
      <v-text-field label="Email" hide-details="auto" v-model="email"></v-text-field>
      <v-btn elevation="4" @click="registerClick">Register</v-btn>
    </div>
  </div>
</template>

<script>
import { mapActions } from "vuex";
export default {
  name: "Registration",
  data() {
    return {
      username: "",
      password: "",
      retypePassword: "",
      email: ""
    };
  },
  methods: {
    ...mapActions(["registerUser"]),
    registerClick() {
      if (this.missingCredentials() === false) {
        if (this.password === this.retypePassword) {
          const user = {
            username: this.username,
            password: this.password,
            email: this.email,
            description: "",
            admin: false
          };
          this.registerUser(user);
          this.username = "";
          this.password = "";
          this.retypePassword = "";
          this.email = "";
        } else {
          this.$alert("Password mismatch");
          this.password = "";
          this.retypePassword = "";
        }
      } else {
        this.$alert("Missing credentials");
      }
    },
    missingCredentials() {
      if (
        this.password === "" ||
        this.username === "" ||
        this.retypePassword === "" ||
        this.email === ""
      ) {
        return true;
      }
      return false;
    }
  }
};
</script>

<style scoped>
.registration-fields {
  width: 400px;
  padding: 50px;
}
</style>
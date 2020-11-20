<template>
  <div>
    <div class="registration-fields">
      <v-text-field
        label="Username"
        hide-details="auto"
        v-model="username"
      ></v-text-field>
      <v-text-field
        label="Password"
        hide-details="auto"
        v-model="password"
      ></v-text-field>
      <v-text-field
        label="Email"
        hide-details="auto"
        v-model="email"
      ></v-text-field>
      <v-btn elevation="4" @click="registerClick">Register </v-btn>
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
      email: "",
    };
  },
  props: {
    msg: String,
  },
  methods: {
    ...mapActions(["registerUser"]),
    registerClick() {
      const user = {
        "username":this.username,
        "password":this.password, 
        "email":this.email,
        "description":"",
        "admin":false
      }
      this.registerUser(user);
      this.username="";
      this.password="";
      this.email="";
    },
  },
};
</script>

<style scoped>
.registration-fields {
  width: 400px;
  padding: 20px;
}
</style>
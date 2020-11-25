<template>
  <div>
    <div class="login-fields">
      <v-text-field
        label="Username"
        hide-details="auto"
        v-model="username"
      ></v-text-field>
      <v-text-field
        label="Password"
        hide-detials="auto"
        v-model="password"
      ></v-text-field>
      <v-btn
      label="Login"
      elevation="4"
      @click="boop"
      >Login</v-btn>
    </div>
  </div>
</template>

<script>
export default {
    data() {
        return {
            username:"",
            password:""
        }
    },
    methods: {
        boop(){
            console.log("boop");
        }
    },
}
</script>

<style scoped>
    .login-fields{
        align-content: center;
        padding: 50px;
        width: 400px;
    }
</style>
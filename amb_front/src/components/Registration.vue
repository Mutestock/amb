<template>
  <div>
    <div class="registration-fields">
      <v-text-field label="Username" hide-details="auto"></v-text-field>
      <v-text-field label="Password" hide-details="auto"></v-text-field>
      <v-text-field label="Email" hide-details="auto"></v-text-field>
      <v-btn elevation="4" @click="registerClick">Register </v-btn>
    </div>
  </div>
</template>

<script>
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
    registerClick() {
      console.log("boop");
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
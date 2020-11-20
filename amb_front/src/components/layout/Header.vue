<template>
  <div>
    <header class="header">
      <div id="nav">
        <router-link to="/">Home</router-link>|
        <router-link to="/about">About</router-link>|
        <router-link to="/user">User</router-link>|
        <router-link to="/registration" v-if="loggedIn"
          >Registration</router-link
        >
        <router-link to="/login" v-if="loggedIn">Log in</router-link>
        <router-link to="/track">Tracks</router-link>
      </div>
      <p v-if="username">{{username}}</p>
    </header>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  data() {
    return {
      username: this.getCurrentUser.username,
      loggedIn: this.hasLoggedInUser,
    };
  },
  methods: {
    ...mapGetters(["hasLoggedInUser", "getCurrentUser"]),
  },
};
</script>

<style scoped>
.header {
  background: #333;
  color: #fff;
  text-align: center;
  padding: 5x;
}
.header a {
  color: #fff;
  padding-right: 10px;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: white;
}

#nav a.router-link-exact-active {
  color: teal;
}
</style>
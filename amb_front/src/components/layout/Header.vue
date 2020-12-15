<template>
  <nav>
    <v-app-bar app dark>
      <v-app-bar-nav-icon class="grey--text" @click="showDrawer =!showDrawer"></v-app-bar-nav-icon>Mutezone
      <router-link to="/">
        <v-icon color="white">mdi-home</v-icon>
      </router-link>
      <router-link to="/about">
        <v-icon color="white">mdi-information</v-icon>
      </router-link>
      <router-link to="/registration" class="account" v-if="!getCurrentUser.username">
        <v-icon color="white">mdi-account-plus</v-icon>
      </router-link>
      <router-link to="/track" v-if="getCurrentUser.username">
        <v-icon color="white">mdi-music-note-plus</v-icon>
      </router-link>
      <router-link to="/login" class="account" v-if="!getCurrentUser.username">
        <v-icon color="white">mdi-login</v-icon>
      </router-link>
      <router-link to="/user" class="account" v-if="getCurrentUser.username">
        <v-icon color="white">mdi-account</v-icon>
      </router-link>
      <v-btn text @click="logoutUser" class="account" v-if="getCurrentUser.username">
        <v-icon color="white">mdi-logout</v-icon>
      </v-btn>
      <v-btn @click="printUsr">Printstuff</v-btn>
    </v-app-bar>
  </nav>
</template>

<script>
import { mapGetters, mapActions, mapState } from "vuex";
export default {
  data() {
    return {
      username: this.getCurrentUser.username
    };
  },
  methods: {
    ...mapActions(["logoutUser"]),
    printUsr() {
      console.log(this.username);
    }
  },
  computed: {
    ...mapGetters(["hasLoggedInUser", "getCurrentUser"]),
    ...mapState(['currentUser'])
  }
  
};
</script>

<style scoped>
.header {
  background: #333;
  color: #fff;
  padding: 5x;
}
.header a {
  color: #fff;
  padding-right: 10px;
}

.account {
  float: right;
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
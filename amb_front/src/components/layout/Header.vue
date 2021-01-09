<template>
  <nav>
    <v-app-bar app dark>
      <v-app-bar-nav-icon
        @click.stop="drawer = !drawer"
        v-if="getCurrentUser.username"
      ></v-app-bar-nav-icon>
      Mutezone
      <router-link to="/">
        <v-icon color="white">mdi-home</v-icon>
      </router-link>
      <router-link to="/about">
        <v-icon color="white">mdi-information</v-icon>
      </router-link>
      <router-link
        to="/registration"
        class="account"
        v-if="!getCurrentUser.username"
      >
        <v-icon color="white">mdi-account-plus</v-icon>
      </router-link>
      <router-link to="/login" class="account" v-if="!getCurrentUser.username">
        <v-icon color="white">mdi-login</v-icon>
        Login
      </router-link>
      <v-btn
        text
        @click="logoutUser"
        class="account"
        v-if="getCurrentUser.username"
      >
        <v-icon color="white">mdi-logout</v-icon>
      </v-btn>
    </v-app-bar>
    <v-navigation-drawer v-model="drawer" absolute bottom temporary>
      <v-list nav dense>
        <v-list-item-group
          v-model="group"
          active-class="deep-purple--text text--accent-4"
        >
          <v-list-item>
            <v-list-item-title>
              <router-link to="/track" v-if="getCurrentUser.username">
                <v-icon color="black">mdi-music-note-plus</v-icon>
              </router-link></v-list-item-title
            >
          </v-list-item>

          <v-list-item>
            <v-list-item-title>
              <router-link to="/user" v-if="getCurrentUser.username">
                <v-icon color="black">mdi-account</v-icon>
              </router-link></v-list-item-title
            >
          </v-list-item>
        </v-list-item-group>
      </v-list>
    </v-navigation-drawer>
  </nav>
</template>

<script>
import { mapGetters, mapActions, mapState } from "vuex";
export default {
  data() {
    return {
      drawer: false,
      group: null,
    };
  },
  methods: {
    ...mapActions(["logoutUser"]),
  },
  computed: {
    ...mapGetters(["hasLoggedInUser", "getCurrentUser"]),
    ...mapState(["currentUser"]),
  },
  watch: {
    group() {
      this.drawer = false;
    },
  },
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

#router-link {
  color:white;
}

#nav a {
  font-weight: bold;
  color: white;
}

#nav a.router-link-exact-active {
  color: teal;
}
</style>
<template>
  <v-app>
    <v-card>
      <h1>{{ currentUser.username }}</h1>
      <h1>{{ currentUser.email }}</h1>
    </v-card></v-app
  >
</template>

<script>
import { mapGetters } from "vuex";
export default {
  name: "UserPage",
  computed: mapGetters({
    currentUser: "getCurrentUser",
  }),
};
</script>
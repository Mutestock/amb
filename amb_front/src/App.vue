<template>
  <v-app>
    <div id="app">
      <title>Mutezone</title>
      <v-main>
        <Header showDrawer />
        <div v-if="showDrawer">
          <RecommendationDrawer showDrawer />
        </div>
        <router-view />
      </v-main>
    </div>
  </v-app>
</template>

<script>
import Header from "@/components/layout/Header";
import RecommendationDrawer from "@/components/layout/RecommendationsDrawer";
export default {
  name: "app",
  components: {
    Header,
    RecommendationDrawer
  },
  props: {
    showDrawer: Boolean
  }
};
</script>

<style>
</style>

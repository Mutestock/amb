<template>
  <v-app>
    <div id="app">
      <title>Mutezone</title>
      <v-main>
        <Header />
        <router-view />
      </v-main>
    </div>
  </v-app>
</template>

<script>
import Header from "@/components/layout/Header";
export default {
  name: "app",
  components: {
    Header
  },
};
</script>

<style>
</style>

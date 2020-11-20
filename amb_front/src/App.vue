<template>
  <div id="app">
    <title>Mutezone</title>
    <Header />
    <router-view />
  </div>
</template>

<script>
import Header from "@/components/layout/Header";
export default {
  name: "app",
  components: {
    Header,
  },
};
</script>

<style>
</style>

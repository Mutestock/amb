<template>
  <v-card height="50">
    <h1>Home page</h1>
    <br>
    <p>Here is a play button just to show that the site can play sound</p>
    <TrackPlay />
    <br>
    <Home />
  </v-card>
</template>

<script>
// @ is an alias to /src
import Home from "@/components/Home.vue";
import TrackPlay from "@/components/TrackPlay.vue";

export default {
  name: "HomePage",
  components: {
    Home,
    TrackPlay,
  },
};
</script>

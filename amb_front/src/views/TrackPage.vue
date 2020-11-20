<template>
  <div class="track">
    <p>Player goes here</p>
    <v-btn elevation="4" @click="createPlayer">Boop</v-btn>
  </div>
</template>


<script>
import { mapActions } from "vuex";
export default {
  name: "TrackPage",
  methods: {
    ...mapActions(["fetchTrackAudio", "createPlayer"])
  },
};
</script>

<style scoped>
.track {
  text-align: center;
}
</style>
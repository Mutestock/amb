<template>
  <div class="track">
    <v-btn elevation="4" @click="createPlayer">Boop</v-btn>
  </div>
</template>


<script>
import { mapActions } from "vuex";
export default {
  name: "TrackPlay",
  methods: {
    ...mapActions(["fetchTrackAudio", "createPlayer"])
  },
};
</script>

<style scoped>
.track {
  text-align: left;
  align-self: left;
}
</style>
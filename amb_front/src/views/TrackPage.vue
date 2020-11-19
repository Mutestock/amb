<template>
  <div class="track">
    <p>Player goes here</p>
  </div>
</template>


<script>
import { mapActions } from "vuex";
export default {
  name: "TrackPage",
  methods: {
    ...mapActions(["fetchTrackAudio", "createPlayer"]),
  },
  created() {
    this.fetchTrackAudio();
  },
};
</script>

<style scoped>
.track {
  text-align: center;
}
</style>
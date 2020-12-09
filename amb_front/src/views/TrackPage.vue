<template>
  <div class="track">
    <TrackUpload />
    <NewTrack />
    <v-btn elevation="4" @click="createPlayer">Boop</v-btn>
  </div>
</template>


<script>
import { mapActions } from "vuex";
import TrackUpload from "@/components/TrackUpload.vue";
import NewTrack from "@/components/NewTrack.vue";
export default {
  name: "TrackPage",
  methods: {
    ...mapActions(["fetchTrackAudio", "createPlayer"])
  },
  components: {
    TrackUpload,
    NewTrack,
  }
};
</script>

<style scoped>
.track {
  text-align: center;
}
</style>
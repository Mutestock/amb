<template>
  <div class="track">
    <TrackUpload />
    <v-btn elevation="4" @click="createPlayer">Boop</v-btn>
  </div>
</template>


<script>
import { mapActions } from "vuex";
import TrackUpload from "@/components/TrackUpload.vue";
export default {
  name: "TrackPage",
  methods: {
    ...mapActions(["fetchTrackAudio", "createPlayer"])
  },
  components: {
    TrackUpload,
  }
};
</script>

<style scoped>
.track {
  text-align: center;
}
</style>
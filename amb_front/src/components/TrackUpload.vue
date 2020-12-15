<template>
  <v-app>
    <div>
      <div class="registration-fields">
        <v-text-field label="Title" hide-details="auto" v-model="title"></v-text-field>
        <v-text-field label="Description" hide-details="auto" v-model="description"></v-text-field>
        <v-text-field label="Credits" hide-details="auto" v-model="credits"></v-text-field>
      </div>
      <div class="upload">
        <v-file-input @change="onFileSelected"></v-file-input>
        <v-btn elevation="4" @click="onUpload">Upload</v-btn>
        <v-progress-linear value="progressPct"></v-progress-linear>
      </div>
    </div>
  </v-app>
</template>

<script>
import axios from "axios";
import { mapGetters } from "vuex";
import * as Tone from "tone";

export default {
  name: "TrackUpload",
  data() {
    return {
      title: "",
      description: "",
      duration: "",
      credits: "",
      progressPct: 0,
      selectedFile: null
    };
  },
  computed: mapGetters({
    currentUser: "getCurrentUser"
  }),
  methods: {
    onFileSelected(event) {
      console.log(event.file);
      const buffer = new Tone.ToneAudioBuffer(event, () => {
        console.log("loaded");
      });
      console.log(buffer.duration);

      this.selectedFile = event;
    },
    onUpload() {
      const fd = new FormData();
      console.log(this.selectedFile);

      fd.append("file", this.selectedFile, this.selectedFile.name);

      fd.append(
        "track",
        `
        {
            "user_id": ${this.getCurrentUser.id},
            "title": ${this.title},
            "description": ${this.description},
            "duration": 0,
            "credits": ${this.credits}
        }
        `
      );

      axios.post(process.env.VUE_APP_BACK_END_HOST + "/api/track/upload", fd, {
        onUploadProgress: uploadEvent => {
          this.pctUpdate(
            Math.round((uploadEvent.loaded / uploadEvent.total) * 100)
          );
        }
      });
    },
    pctUpdate(pct) {
      this.progressPct = pct;
    }
  }
};
</script>
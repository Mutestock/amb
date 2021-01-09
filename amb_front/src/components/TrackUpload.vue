<template>
  <v-app>
    <v-card elevation="7" class="card">
      <div class="registration-fields">
        <v-text-field
          label="Title"
          hide-details="auto"
          v-model="title"
        ></v-text-field>
        <v-text-field
          label="Description"
          hide-details="auto"
          v-model="description"
        ></v-text-field>
        <v-text-field
          label="Credits"
          hide-details="auto"
          v-model="credits"
        ></v-text-field>
      </div>
      <div class="registration-fields">
        <v-file-input @change="onFileSelected"></v-file-input>
        <v-btn elevation="4" @click="onUpload">Upload</v-btn>
        <v-progress-linear value="progressPct"></v-progress-linear>
      </div>
    </v-card>
  </v-app>
</template>

<script>
import axios from "axios";
import { mapGetters } from "vuex";
import * as Tone from "tone";
import store from "../store";

export default {
  name: "TrackUpload",
  data() {
    return {
      title: "",
      description: "",
      duration: 0,
      credits: "",
      progressPct: 0,
      selectedFile: null,
    };
  },
  computed: mapGetters({
    currentUser: "getCurrentUser",
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
      if(this.title ==="" || this.credits==="")[
        this.$alert("Neither title nor credits can be empty")
      ]
      else if (this.selectedFile !== null) {
        const fd = new FormData();

        fd.append("file", this.selectedFile, this.selectedFile.name);
        fd.append(
          "track",
          `
        {
            "user_id": ${store.getters.getCurrentUser.id},
            "title": "${this.title}",
            "description": "${this.description}",
            "duration": 0,
            "credits": "${this.credits}"
        }
        `
        );
        axios.post(
          process.env.VUE_APP_BACK_END_HOST + "/api/track/upload",
          fd,
          {
            onUploadProgress: (uploadEvent) => {
              this.pctUpdate(
                Math.round((uploadEvent.loaded / uploadEvent.total) * 100)
              );
            },
          }
        );
        this.title = "";
        this.description = "";
        this.duration = 0;
        this.credits = "";
        this.selectedFile = null;
        this.$alert("Ok");
      } else {
        this.$alert("Please select a file to upload");
      }
    },
    pctUpdate(pct) {
      this.progressPct = pct;
    },
  },
};
</script>

<style scoped>
.card {
  width: 50%;
}
.registration-fields {
  padding: 20px;
}
</style>
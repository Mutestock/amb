<template>
  <div>
    <div class="upload">
      <v-file-input @change="onFileSelected"></v-file-input>
      <v-btn elevation="4" @click="onUpload">Upload</v-btn>
    </div>
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "TrackUpload",
  data() {
    return {
      selectedFile: null
    };
  },
  methods: {
    onFileSelected(event) {
      console.log(event);
      this.selectedFile = event;
    },
    onUpload() {
      const fd = new FormData();
      console.log("stuff");
      console.log(this.selectedFile);

      fd.append("file", this.selectedFile, this.selectedFile.name);
      console.log("thing");

      fd.append(
        "track",
        `
        {
            "user_id": 1,
            "title": "test-track-title",
            "description": "test-track-description",
            "duration": 69,
            "credits": "test-track-credits"
        }
        `
      );
      console.log("shite");

      axios.post(process.env.VUE_APP_BACK_END_HOST + "/api/track/upload", fd, {
        onUploadProgress: uploadEvent => {
          console.log(
            "Upload progress: " +
              Math.round((uploadEvent.loaded / uploadEvent.total) * 100) +
              "%"
          );
        }
      });
    }
  }
};
</script>
<template>
  <div>
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
        label="Duration"
        hide-details="auto"
        v-model="duration"
      ></v-text-field>
      <v-text-field
        label="Credits"
        hide-details="auto"
        v-model="credits"
      ></v-text-field>
    </div>
  </div>
</template>

<script>
export default {
  name: "NewTrack",
  data() {
    return {
      title: "",
      description: "",
      duration: "",
      credits: "",
    };
  },
};
</script>
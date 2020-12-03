<template>
  <div>
    <v-alert
      dense
      v-model="alert"
      border="left"
      type="error"
      color="grey"
      text-color="yellow"
      dismissible
    >Please insert your credentials in all of the fields</v-alert>
  </div>
</template>

<script>
export default {
  data () {
    return{
      alert:false
    }
  }
};
</script>
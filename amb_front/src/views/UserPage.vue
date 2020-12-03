<template>
  <div>
    <div class="user">
    </div>
    <h1>{{currentUser.username}}</h1>
    <h3></h3>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  name: "UserPage",
  computed: mapGetters({
    currentUser:'getCurrentUser'})
};
</script>
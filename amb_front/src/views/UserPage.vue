<template>
  <div>
    <User msg="User" />
    <div class="user">
      <div v-for="user in allUsers" :key="user.id">{{ user.username }}</div>
    </div>
  </div>
</template>

<script>
import User from "@/components/User.vue";
import { mapGetters, mapActions } from "vuex";
export default {
  name: "UserPage",
  methods: {
      ...mapActions(['fetchUser'])
  },
  computed: mapGetters(["allUsers"]),
  created() {
      this.fetchUser();
  },
  components: {
    User
  }
};
</script>
<template>
  <div>
    <header class="header">
      <div id="nav">
        <router-link to="/">Home</router-link>|
        <router-link to="/about">About</router-link>|
        <router-link to="/user">User</router-link>|
        <router-link to="/registration">Registration</router-link>
        <router-link to="/track">Tracks</router-link>
      </div>
    </header>
  </div>
</template>

<script>
export default {};
</script>

<style scoped>
.header {
  background: #333;
  color: #fff;
  text-align: center;
  padding: 5x;
}
.header a {
  color: #fff;
  padding-right: 10px;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: white;
}

#nav a.router-link-exact-active {
  color: teal;
}
</style>
<template>
    <div>
        <User msg="User" />
    </div>
</template>

<script>
    import User from "@/components/User.vue"
    export default {
        name: "UserPage",
        components:{
            User
        }
    }
</script>
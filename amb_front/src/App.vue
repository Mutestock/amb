<template>
  <div id="app">
    <header>
      <h1>sound</h1>
    </header>
    <main>
      <section class="player">
        <h2 class="song-title">{{current.title}} - <span>{{current.artist}}</span></h2>
        <div class="control">
          <!-- <button class="prev" @click="previous">Previous</button> -->
          <button class="play" v-if="!isPlaying" @click="play">Play</button>
          <button class="pause" v-else @click="pause">Pause</button>
          <!-- <button class="next" @click="next">Next</button> -->
        </div>
      </section>
      <section class="playlist">
        <h3>Playlist</h3>
        <button v-for="song in songs" :key="song.src"
          @click="play(song)" :class="(song.src == current.src) ? 'song playing' : 'song'">
            {{song.title}}--{{song.artist}}
        </button>
      </section>
    </main>
  </div>
</template>

<script>
//import HelloWorld from './components/HelloWorld.vue'
import axios from 'axios';

export default {
  name: 'App',
  data () {
    return {
      current:{},
      index: 0,
      isPlaying: false,
      songs: [
        {
          title: "bird",
          artist: "derp",
          src: "",
        },
        //{
        //  title: "creak",
        //  artist: "herb",
        //  src: require('../../../resources/AMB_creak.wav')
        //}
      ],
      player: new Audio()
    }
  },
  methods:{
    play(song) {
      if (typeof song.src != "undefined"){
        this.current=song;
        this.player.src = this.current.src;
      }
      this.player.play();
      this.isPlaying = true;
    },
    pause() {
      this.player.pause();
      this.isPlaying=false;
    },
    //next(){
    //  this.index++;
    //  if (this.index> this.songs.length -1){
    //    this.index = 0;
    //  }
    //  this.current= this.songs[this.index];
    //  this.play(this.current);
    //},
    //previous(){
    //  this.index--;
    //  
    //  if (this.index < 0){
    //    this.index = this.songs.length-1;
    //  }
    //  this.current= this.songs[this.index];
    //  this.play(this.current);
    //}
  },
  created () {
    axios.get('mutezone.site:8000/files/bird.wav')
      .then(res => this.current.src = res.data)
      .catch(err => console.log(err));
    this.player.src = this.current.src;
    this.player.play();
  }
  //components: {
  //  HelloWorld
  //}
}
</script>

<style>
*  {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
body {
  font-family: sans;
}
header {
  display:flex;
  justify-content:center;
  align-items:center;
  padding: 15px;
  background-color: #212121;
  color: #FFF;
}
</style>

<template>
  <v-app>
    <v-card elevation="7" class="stick_out" >
      <p>
        Welcome to the ambience project This project is primarily created as
        part of the finals for the Computer Sciences education.
      </p>
      <p></p>
      <p>
        The idea is to create ambience or background white noise, but with
        dynamic, adjustable and
      </p>
      <p>
        subscribable sounds. These ambient compositions are often used for
        roleplaying purposes, often with DnD. Many people use them as background
        noise while working or studying as well.
      </p>
    </v-card>
    <br />
    <div align="center">
      <iframe
        width="420"
        height="315"
        src="https://www.youtube.com/embed/kxqJuc1HHbg"
      >
      </iframe>
    </div>
    <br />
    <v-card elevation="7" class="stick_out">
      <p>The file above can be disected into several different elements.</p>
      <p>
        First is the background humming. This would be an audio file of anything
        from 10 secs to several minutes on loop.
      </p>
      <p>
        Second could be the water droplets. Presumably these droplets would, in
        an IRL scenario, come from a random position in the cave, and with
        various intensity.
      </p>
      <p>
        In the mix above, this is hard placed all the sounds are constant. This
        means that the 45 min file is itself looped somewhere, e.g. 7 minutes,
        and you'd be able to hear where that loop is with a sharp ear.
      </p>
      <p>
        With this website, you'll be able to create a configurartion file which
        contain random intervals, random volumes, random panning, etc, all
        within boundaries that you choose yourself.
      </p>
      <br />
      <p>So lets say I want to make a composiition to simulate a forest.</p>
      <p>
        First I might want to add the sound of leaves from trees, so I add a 4
        min loop of leaves (or set of leaf sounds to mask the loop)
      </p>
      <p>Next I might add the sound of birds.</p>
      <p>
        These bird should be able to come from all sides around me, like birds
        usually do. I set pan-X and pan-Y to 0-360 random to signify random
        relative position.
      </p>
      <p>
        To simulate distance from the listener, I add random percentage volume
        14-72%
      </p>
      <p>
        They won't be making sounds all the time, so I set the interval to be
        random within e.g. 36-81 seconds.
      </p>
      <p>
        All these sounds are saved as configurations which are added to the
        playlist/composition that you've made.
      </p>
      <p>
        This means you can add the same sound multiple times with different
        configurations.
      </p>
      <p>You'll be able to listen to the compositions of others.</p>
      <p>You'll be able to upload sounds yourself.</p>
      <p>
        You'll be able to use your sounds and the sounds other people have
        uploaded.
      </p></v-card
    >
  </v-app>
</template>

<style scoped>
.multiline {
  white-space: pre-wrap;
}
.stick_out {
  padding-left: 50px;
  width: 87%;
  padding: 20px;
}
</style>
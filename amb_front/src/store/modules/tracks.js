import axios from 'axios';
import * as tone from 'tone';
import { Tone } from 'tone/build/esm/core/Tone';

const state = {
    track_audio: null
};

const getters = {};

const actions = {
    async fetchTrackAudio() {
        const response = await axios.get(
            process.env.VUE_APP_BACK_END_HOST + "/files/bird.wav"
        );
        console.log(response.data);
    },

    async createPlayer() {
        const player = new Tone.player({
            url: process.env.VUE_APP_BACK_END_HOST + "/files/bird.wav",
            loop: true,
            autostart: true,
        });
        Tone.loaded().then(() => {
            player.start();
        });
    }
};

const mutations = {};

export default {
    state,
    getters,
    actions,
    mutations
};
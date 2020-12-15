import axios from 'axios';
import * as Tone from 'tone'


const state = {
    player: null,
    track_to_upload: null,
};

const getters = {};

const actions = {
    async fetchTrackAudio() {
        const response = await axios.get(
            process.env.VUE_APP_BACK_END_HOST + "/files/bird.wav"
        );
        console.log(response.data);
    },

    async createPlayer({ commit }) {
        const player = new Tone.Player({
            url: process.env.VUE_APP_BACK_END_HOST + "/files/bird.wav"
        }).toDestination();
        Tone.loaded().then(() => {
            player.start();
        });
        commit("setPlayer", player)
    },

};

const mutations = {
    setPlayer: (state, player) => (state.player = player),
    set_track_to_uplooad: (state, track_to_upload) => (state.track_to_upload = track_to_upload)
};

export default {
    state,
    getters,
    actions,
    mutations
};
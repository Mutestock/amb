import axios from 'axios';
import * as Tone from 'tone'
import { trackService } from "../../_services/user_service";


const state = {
    player: null,
    track_to_upload: null,
    trackList:[]
};

const getters = {
    getTrackList: (state) =>state.trackList
};

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

    async fetchAllTracks({ commit }) {
        const response = await trackService.list()
        commit("SET_trackList", response)
    }


};

const mutations = {
    setPlayer: (state, player) => (state.player = player),
    setTrackToUpload: (state, track_to_upload) => (state.track_to_upload = track_to_upload),
    SET_trackList: (state, track_list) => (state.track_list = track_list),
};

export default {
    state,
    getters,
    actions,
    mutations
};
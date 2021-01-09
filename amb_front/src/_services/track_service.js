import axios from "axios";

const USER_URL_BASE = process.env.VUE_APP_BACK_END_HOST + "/api/track";

export const trackService = {
    list
}

function list(){
    return axios.get('USER_URL_BASE').then(handleResponse);
}

function handleResponse(response) {
    return response.text.then((text) => {
      const data = text && JSON.parse(text);
      if (!response.ok) {
        if (response.status === 401) {
          // Statuscode 401 == Unauthorized
          logout();
          location.reload();
        }
        const error = (data && data.message) || response.statusText;
        return Promise.reject(error);
      }
      return data;
    });
  }
  
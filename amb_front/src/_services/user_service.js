import { auth } from "../_helpers";
import axios from 'axios';

const blackListedSymbolsPattern = /[&\\/<>'=?!{}]/;

function checkEmptyFields(input){
    if (!(input.is_empty())){
        return input
    }
    else {
        console.log("Empty input fields. Insert error message here");
    }
}

function checkIllegalCharacters(input) {
    if (input.replace(blackListedSymbolsPattern, "").length() == input.length()) {
        return input;
    }
    else {
        console.log("Use of blacklisted symbol. Insert error message here");
    }
}


export const userService = {
    login,
    logout,
    register,
    update,
    delete: _delete
};

const USER_URL_BASE = process.env.VUE_APP_BACK_END_HOST + "/api/user";

async function login(username, password) {
    checkIllegalCharacters(username);
    checkIllegalCharacters(password);
    checkEmptyFields(username);
    checkEmptyFields(password);

    const requestOptions = {
        method: 'POST',
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ username, password })
    };

    return await axios.post(
        USER_URL_BASE + "/login", requestOptions)
        .then(handleResponse);
}

//Sunply removes the item in the local storage with the key 'user'
function logout() {
    localStorage.removeItem('user');
}

async function register(user) {
    checkIllegalCharacters(user.username);
    checkIllegalCharacters(user.password);
    checkEmptyFields(user.username);
    checkEmptyFields(user.password);

    const requestOptions = {
        method: 'POST',
        headers: { "content-type": "application/json" },
        body: JSON.stringify(user)
    };

    return await axios.post(USER_URL_BASE, requestOptions)
        .then(handleResponse);
}

function update(user) {
    const requestOptions = {
        method: 'PUT',
        headers: { ...auth(), 'content-type': 'application/json' },
        body: JSON.stringify(user)
    };

    return axios(
        USER_URL_BASE, requestOptions
    )
}


function _delete(id) {
    const requestOptions = {
        method: 'DELETE',
        headers: auth()
    };

    return axios(
        USER_URL_BASE + `${id}`, requestOptions
            .then()
    )
}

function handleResponse(response) {
    return response.text()
        .then(text => {
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
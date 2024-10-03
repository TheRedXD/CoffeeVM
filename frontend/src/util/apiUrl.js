export let apiUrl;
if (!import.meta.env.PROD) {
    apiUrl = "http://localhost:8080"
} else {
    apiUrl = location.origin;
}

export function buildAPIUrl(route) {
    return apiUrl + "/api" + route;
}

export default {
    apiUrl,
    buildAPIUrl
}
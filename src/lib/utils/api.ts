import axios from "axios"

const base = process.env.NODE_ENV == "production" ? "https://business.api.studiotwofour.com" : "http://localhost:8787"

const api = axios.create({
    baseURL: base,
})

export default api
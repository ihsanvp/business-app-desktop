const LOCATION = ".app/updater.json"

const { Octokit } = require("octokit")
const axios = require("axios")
const path = require("path")
const fs = require("fs")


const octokit = new Octokit({
    auth: process.env.GITHUB_AUTH_TOKEN
})

const AssetMap = {
    "darwin-x86_64": {
        signature: ".tar.gz.sig",
        asset: ".app.tar.gz"
    },
    "linux-x86_64": {
        signature: ".AppImage.tar.gz.sig",
        asset: ".AppImage.tar.gz"
    },
    "windows-x86_64": {
        signature: ".nsis.zip.sig",
        asset: ".nsis.zip"
    }
}

async function run() {
    const release = await octokit.request("GET /repos/{owner}/{repo}/releases/{release_id}", {
        owner: "ihsanvp",
        repo: "business-app-desktop",
        release_id: process.env.RELEASE_ID,
        headers: {
            'X-GitHub-Api-Version': '2022-11-28'
        }
    })

    const final = {
        version: getVersion(release.data.tag_name),
        notes: release.data.body_text,
        pub_date: release.data.published_at,
        platforms: {}
    }

    const data = await Promise.all(Object.keys(AssetMap).map(key => {
        const identifier = AssetMap[key]
        const urlAsset = release.data.assets.find(a => a.name.endsWith(identifier.asset))
        const signatureAsset = release.data.assets.find(a => a.name.endsWith(identifier.signature))

        return getAssetData(urlAsset, signatureAsset)
    }))

    Object.keys(AssetMap).forEach((key, i) => {
        final.platforms[key] = data[i]
    })

    await saveToFile(final)
}

async function getSignature(asset) {
    const res = await axios.get(asset.browser_download_url, {
        responseType: "text"
    })
    return res.data
}

async function getAssetData(urlAsset, sigAsset) {
    return {
        signature: await getSignature(sigAsset),
        url: urlAsset.browser_download_url
    }
}

async function saveToFile(data) {
    return new Promise((resolve, reject) => {
        const parts = path.parse(LOCATION)

        if (!fs.existsSync(parts.dir)) {
            fs.mkdirSync(parts.dir, { recursive: true })
        }

        fs.writeFileSync(LOCATION, JSON.stringify(data))
        resolve()
    })
}

function RFC3339DateString(d) {
    function pad(n) { return n < 10 ? '0' + n : n }
    return d.getUTCFullYear() + '-'
        + pad(d.getUTCMonth() + 1) + '-'
        + pad(d.getUTCDate()) + 'T'
        + pad(d.getUTCHours()) + ':'
        + pad(d.getUTCMinutes()) + ':'
        + pad(d.getUTCSeconds()) + 'Z'
}

function getVersion(tagName) {
    const parts = tagName.split("-v")
    const version = parts[parts.length - 1]

    return `v${version}`
}


run().catch(console.log)
module.exports = async ({ github, context, fetch, core }) => {
    async function getSignature(asset) {
        const res = await fetch(asset.browser_download_url)
        const data = await res.text()
        return data
    }

    async function getAssetData(urlAsset, sigAsset) {
        return {
            signature: await getSignature(sigAsset),
            url: urlAsset.browser_download_url
        }
    }


    function getVersion(tagName) {
        const parts = tagName.split("-v")
        const version = parts[parts.length - 1]

        return `v${version}`
    }

    console.log("getting release...")
    const release = await github.rest.repos.getRelease({
        owner: context.repo.owner,
        repo: context.repo.repo,
        release_id: process.env.RELEASE_ID,
    })

    const data = {
        version: getVersion(release.data.tag_name),
        notes: release.data.body_text,
        pub_date: release.data.published_at,
        platforms: {}
    }

    console.log("creating platforms data...")
    const platforms = await Promise.all(Object.keys(AssetMap).map(key => {
        const identifier = AssetMap[key]
        const urlAsset = release.data.assets.find(a => a.name.endsWith(identifier.asset))
        const signatureAsset = release.data.assets.find(a => a.name.endsWith(identifier.signature))

        return getAssetData(urlAsset, signatureAsset)
    }))

    Object.keys(AssetMap).forEach((key, i) => {
        data.platforms[key] = platforms[i]
    })

    core.setOutput("updaterData", JSON.stringify(data))
}
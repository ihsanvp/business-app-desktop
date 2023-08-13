const { Octokit } = require("octokit")

async function run() {
    const github = new Octokit({ auth: process.env.GITHUB_AUTH_TOKEN })
    const args = process.argv.slice(2)

    if (args.length != 1) {
        throw new Error("'tag_name' not provided")
    }

    const tag_name = args[0]
    const release = await github.rest.repos.getReleaseByTag({
        owner: "ihsanvp",
        repo: "business-app-desktop",
        tag: tag_name
    })

    console.log(release.data.id)
}

run().catch(console.log)
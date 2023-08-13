#### Step 1 
Manually check app version in `package.json`

#### Step 2
Create new branch with name `v-x.x.x`

- Click `branches` | above code
- Click `New Branch` | top right
- Enter branch name

#### Step 3
Create new draft release

- Click `Releases` | right side of code
- Click `Draft a new Release` | top right
- Enter Title
- Enter Tag
- Enter Release Notes
- Click `Generate release notes` | above markdown editor
- Uncheck `Set as the latest release` | bottom
- Click `Publish release` | bottom

#### Step 4
Get release id

-   ```
    gh release list
    ```
-   ```
    gh release view {tag_name} --json id
    ```

- Take the release id

#### Step 4
Start `Publish` action

- Click `Actions` | top
- Click `Publish New Release` | left sidebar
- Click `Run workflow`
- Enter the `release_id` from previous step
# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## Branching

/stable
    /staging
        /release
            / feature some_thing                            /bugfix bug-#ticketno
                / dev random_feat1  /test random_feat1


stable : this branch is the main branch of repo , all stable staging branches are merged into this . This is what will trigger artifact creation and update release page

staging : this branch will be used for triggering artifacts creation for testing purposes for end users (beta test users)

release : this branch is version branch , release branch can have multiple feature branch . Something similar to how , a new release app will bring new feature . And it can also have multiple bugfix branches . Release can ofcourse have multiple bug fix

feature : feature branch will have multiple dev branches and dev's parallel test branch. Multiple dev branches can form a single feature .

bugfix-#tickernumber : Is for picking up bug tickets from github issues . and solving it for new release

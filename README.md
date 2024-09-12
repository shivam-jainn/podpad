# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## Branching

/stable
    /staging-v{int-int}
        /release-v{int-int-int}
            / feature some_thing                            /bugfix bug-#ticketno
                / dev random_feat1  /test random_feat1


stable : this branch is the main branch of repo , all stable staging branches are merged into this . This is what will trigger artifact creation and update release page

staging : this branch will be used for triggering artifacts creation for testing purposes for end users (beta test users)

release : this branch is version branch , release branch can have multiple feature branch . Something similar to how , a new release app will bring new feature . And it can also have multiple bugfix branches . Release can ofcourse have multiple bug fix

feature : feature branch will have multiple dev branches and dev's parallel test branch. Multiple dev branches can form a single feature .

bugfix-#tickernumber : Is for picking up bug tickets from github issues . and solving it for new release


### Example

/stable
/staging-v0.1
/release-v0.0.1 [each release 0.1.0 == staging-v0.1] [10 release cycles = 1 staging cycle]
    /feature-[filesystem-sidebar]
        /dev-[rust-fs-access] /test-[rust-fs-access]



## Folder Structure 

/src
    /{page}
        /components
            /....
        /page.tsx
/src-tauri
    /src
        /{feature}

## How to contribute?

1. Fork this repo
2. Pick an issue
3. Try to solve it and ask doubts if any
4. Make commits using `conventional commits` (It's in VSCode extension marketplace)
5. Make a PR to respective feature branch (the feature branch would be given in the ticket)
# Contribution Guide
## Who can contribute
Only members of the ASU CodeDevils organization who have applied through 
[the website](https://www.codedevils.org/projects/gamedev) are allowed to contribute to this repo.

## Calling dibs on an issue
Before working on an issue, be sure to comment on that issue to mention that you
want to work on it, and a project lead will assign the issue to you. You will be
expected to show satisfactory progress within a week of being assigned an issue.
If not enough progress was made, you will be unassigned from the issue so that
other members can try their hand at it.

>[!IMPORTANT]
> Pull requests made to fix an issue that you are not assigned to will be
> immediately closed.

## One issue at a time
You are only allowed to tackle one issue at a time. If you wish to work on a
different issue, let us know that you no longer want to work on the currently
assigned issue.

>[!NOTE]
> If it is suspected that you are working on issues before being assigned to
> them, i.e. you get assigned an issue and come up with a fully working solution
> moments later to "speedrun" contributions, the project lead reserves the right
> to limit the rate at which you can be assigned issues.

## Branch naming
When naming your branch, follow the following convention:
`<TYPE>/<ISSUE-ID>/<SHORT-DESCRIPTION>`

### Types
- Enhancements and new features: `feature`
- Bug fix: `bugfix`
- Assets: `asset`
- Refactors: `refactor`
- Documentation: `docs`
- Updating dependencies: `update`

### Issue ID
The issue ID associated with the issue you are working on. If there are no issues, then please create one first.

### Short description
Use kebab-case and keep it as short, but descriptive as possible.

### Examples
You are working on issue #10, which is to create a new double jump ability, then you would name your branch:\
`feature/10/double-jump`

You are creating a new spritesheet for a new enemy, as instructed in issue #24:\
`asset/24/new-enemy-spritesheet`

## Commit naming and descriptions
Keep the commit name short and to the point, and use present tense. Within the description, include any relevant information on why you made specific decisions. 

### Examples
You are working on creating a new double jump ability:
#### Good commit
**Commit name:** add double jump\
**Commit desc:** Created a new component to keep track of current jump count, as opposed to using the player state, as it felt more appropriate for an ECS architecture.

#### Bad commit
**Commit name:** added double jump\
**Commit desc:** I added a double jump ability

#### Very bad commit
**Commit name:** work in progress commit\
**Commit desc:** The player can jump too many times, just gotta fix that and then should be good to go!

### Good practices
When submitting your work through a pull request, ensure that commits always represents a fully working piece of code.
If you have any intermediate commits, which can be pretty common when working on
an issue, make sure you squash all intermediate commits. Generally, you should
only be submitting a single commit in your pull request, unless you are working
on a really big issue.

## Plugins and bundles
When you code, try to use bevy plugins and bundles whenever possible. Use Rust modules to isolate these plugins and bundles.

## Credits
If you have added assets from a 3rd party, ensure you also update the [`CREDITS.md`]("./CREDITS.md") file with the necessary information.

## Before submitting pull request
Run the following commands from the root directory of the project and fix any issues they find:
```bash
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
cargo fmt --all -- --check
typos
```
Issues found by `cargo fmt --all -- --check` can be fixed by running `cargo fmt --all`. Issues found by `typos` can usually be fixed with `typos -w`.

If you do not have `typos` installed, you can install it first by running the
following command:
```bash
cargo install typos-cli
```

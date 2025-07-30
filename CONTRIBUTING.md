#  Contributing to Zoea Open Source

Welcome! 🎉 We're thrilled you're considering contributing to Zoea Open Source.
This repository is designed to help beginners get comfortable with GitHub, contributions, and open source collaboration.

This document will walk you through how to get started and how you can contribute. 
Whether you're fixing typos, writing code, or improving docs, you're making an impact.

## 📁 Repository Structure

Here’s what you’ll find at the root of this repo:

```yaml
zoea-opensource/
│
├── guides/      ← Markdown guides to help you learn how to contribute
├── solutions/   ← Contributions from users: code, tutorials, experiments, etc.
├── translations/   ← Guide Translations to help you read in your preferred language.

```

## Ways You Can Contribute

You can contribute in many ways, including:

### 📝 Guides

Add or improve markdown files in the guides/ folder:

- How-tos (e.g., “How to Make Your First PR”)
- Git tutorials
- Open source contribution tips
- Anything that can help beginners get started
- Translations

### 💡 Solutions

In the solutions/ folder, you can:

- Submit code snippets, mini projects, or experiments
- Fix simple bugs or improve existing code
- Resolve and explain merge conflicts (great learning opportunity!)
- Add example PRs for learning purposes

You can even submit creative solutions to example challenges we'll post.
### 🛠 Other Contributions

- Fix typos or grammar issues in markdown files
- Improve file organisation
- Help with formatting and structure of documentation
- Add helpful links and resources

## Getting Started

> [!NOTE]<br>
> POV: Please read the following simple steps on how to contribute. This will make life easier and will avoid wasting time on things which are not requested. ✨

## Steps to follow

Open a copy of this page on another tab to help you follow up.

<details>
<summary>
Step 1: Star The Repo (Optional)
</summary>
<br>
  
  - Star the repo by pressing the topmost-right button to start your wonderful journey

||
|-|
|<img width="453" height="63" alt="Image" src="https://github.com/user-attachments/assets/4c69730c-fcf8-4182-9bbd-d3003a2a0360" />|

</details>

---

<details>
<summary>
Step 2: Fork this repo
</summary>
<br>
  
- On the [GitHub page for this repository](https://github.com/rezzcode/zoea-opensource), click on the Button ["**Fork**"](https://github.com/rezzcode/zoea-opensource/fork).

||
|-|
|<img width="453" height="61" alt="Image" src="https://github.com/user-attachments/assets/f790f171-c91e-4aa6-9b83-42cc133195ca" />|

</details>

---

<details>
<summary>
Step 3: Clone the forked repo to your machine
</summary>

<br>

- **Method 1:** GitHub Desktop

> ⚠️ **NOTE:** If you're not familiar with Git, using **GitHub Desktop Application** is a better start. If you choose this method, make sure to download it before continuing reading.
>
> ❗❗ Access link to download [**here**](https://desktop.github.com).

- **Method 2:** Git

Clone the forked repository. Open git bash and type:

```bash
git clone https://github.com/<your-github-username>/zoea-opensource.git
cd zoea-opensource
git config --global user.name "<your GitHub user name>" && git config --global user.email "your GitHub primary email"
```

> This makes a local copy of the repository in your machine.
>
> ⚠️ **Replace \<your-github-username\>!**

Learn more about [forking](https://help.github.com/en/github/getting-started-with-github/fork-a-repo) and [cloning a repo](https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/cloning-a-repository).

</details>

---

<details>
<summary>
Step 4: Create a new branch for your changes
</summary>

<br>

Always keep your local copy of the repository updated with the original repository.
Before making any changes and/or in an appropriate interval, follow the following steps:

- **Method 1:** GitHub Desktop

Learn more about how to create a new branch [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/making-changes-in-a-branch/managing-branches#creating-a-branch) and how to fetch and pull origin from/to your local machine [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/keeping-your-local-repository-in-sync-with-github/syncing-your-branch).

Learn more about how to fetch and pull origin from/to your local machine using **GitHub Desktop** [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/keeping-your-local-repository-in-sync-with-github/syncing-your-branch).

- **Method 2:** Git

Run the following commands **_carefully_** to update your local repository

```bash
# If you cloned a while ago, get the latest changes from upstream
git checkout main
git pull upstream main

# Make a feature branch (Always check your current branch is up to date
# before creating a new branch from it to avoid merge conflicts)
git checkout -b <your-new-branch-name>

#
```

</details>

---

<details>
<summary>
Step 5: Ready, Set, Go...
</summary>

<br>

- Once you have completed these steps, you are ready to start contributing to the project and creating **pull requests**.

### Translation Contribution
- Ensure that you read [Translation Guide for more info on this](https://github.com/rezzcode/zoea-opensource/blob/main/Translations/README.md)
- Once you are done with your translation, edit the [Translation Guide readme](https://github.com/rezzcode/zoea-opensource/blob/main/Translations/README.md) 
to include the language you have translated to and the lint to the file.

**How to do this***
In the [Translation Guide readme](https://github.com/rezzcode/zoea-opensource/blob/main/Translations/README.md), the language of translation will be included inside a `[]`bracket,
 and the link inside a `()`

So it should look something like:
```yaml
[language](link)
```

### Code Snippet Contribution
- Create a folder if you want to contribute a new code snippet in a different language.
  > 1. The folder name **Must** be the name of the language, and the files **must** be relevant to the code you want to submit. <br>
  > 2. The folder created **Must** be added to the path: `Solutions/Snippets`, followed by the language name directory, then the file <br>
  > Example: If the new snippet is in Java programming, the path should be as follows. `Solutions/Snippets/Java/your-file-name.java`
- If your language snipped directory already exists, then add your file to the existing directory

### Guide Contribution
- Ensure your guide is meaningful and beginner-friendly
- Don't forget to add a `README.md` in your folder.

* **Method 1:** GitHub Desktop

Learn more about how to make pull requests from your local machine using **GitHub Desktop** to the main repo [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/working-with-your-remote-repository-on-github-or-github-enterprise/viewing-a-pull-request-in-github-desktop).

- **Method 2:** Git

Add and commit with a clear message using `git add`, `git commit`:

```bash
git add -A
git commit -m "<your message>"
```

Push the code _to your repository_.

```bash
git push -u origin <branch-name>
```

Make sure you have no conflicts. 🙂 🙂

</details>

---

<details>
<summary>
Step 6: Open a Pull Request (PR)
</summary>

<br>

Go to the GitHub page of _your fork_, and **make a pull request**:

Read more about pull requests on the [GitHub help pages](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

- Now wait, until _your Pull Request_ is approved! If there are any conflicts, you will get a notification.

</details>

📌 Guidelines

- ✅ Be kind and respectful – especially to beginners
- ✅ Keep content beginner-friendly and easy to understand
- ✅ Follow markdown best practices for `guides/`
- ✅ Organise code clearly for `solutions/`
- ✅ Include comments where necessary in code
- ✅ Keep PRs focused – one topic/change per PR
- ✅ Feel free to ask for help via Issues or Discussions

## Examples

Here are some example guides you can pick to get started with the contribution.

|[Fixing Merge Conflicts](https://github.com/rezzcode/zoea-opensource/tree/main/Guides/Conflicts_Guide)|[Adding name to Contributors.md](https://github.com/rezzcode/zoea-opensource/tree/main/Guides/Names)||
|-|-|-|
||||

## 🚧 What We're Still Planning

We're currently working on:

- Creating beginner-friendly issues for you to tackle
- Templates for code challenges
- Directory structure best practices

Have ideas? Open an [issue](https://github.com/rezzcode/zoea-opensource/issues) and share it!

## Need Help?

No worries, this is a learning space. If you're unsure how to start, visit the `guides/` directory or ask a question in the Discussions tab.

## Thank You

Thank you for being part of Zoea Open Source. Your contribution—big or small is helping someone else grow. That’s the magic of open source.

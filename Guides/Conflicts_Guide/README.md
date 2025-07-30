# 🧩 Practice Resolving Merge Conflicts (GitHub Guide)

Welcome to your first hands-on merge conflict resolution task! This guide walks you through resolving a **merge conflict via GitHub** step by step.

> 💬 **Note**: This is a GitHub-focused guide. A CLI (local terminal) version is coming soon — feel free to contribute it!


## 📖 Open This Guide in a New Tab
Keep this page open side-by-side while you work.

---

## 🐛 Step 1: Create an Issue

1. Go to the repo’s [**Issues**](https://github.com/rezzcode/zoea-opensource/issues) tab.
2. Click **"New Issue"**.
3. Choose the template `Practice: Resolving Merge Conflicts`.
4. Follow the instructions provided.
5. Select the dropdown to choose one:
   - "🧠 Text Conflict"
   - "💻 Code Conflict"
6. (Optional) Go to [`repo/tasks/conflicts/`](../../tree/main/Tasks/Conflicts/) and choose a file to work on.
   - Enter the **full name** of the file in the "File Name" section. Example: `example.md` or `app.js` don't enter files like `example2.md` or `example3.md`
7. Click **Create Issue**.

> ⚠️ Now wait on the issue page. You will get assigned a **custom branch name** for you to use. It’ll appear as a comment.


## 🍴 Step 2: Fork the Repository

1. On the main repo page, click **Fork** (top-right).
2. In the fork dialogue, **uncheck** `Copy only the main branch`. 
   _✅ This ensures you copy the conflict branch too._

|  Image Illustration  |
|-|
|<img width="758" height="385" alt="Image" src="https://github.com/user-attachments/assets/d80a740b-6b07-4ace-be05-db9c7b32d177" />|

---

## 🔀 Step 3: Start the Pull Request

1. Go to your **forked repo**.
2. Click **Pull Requests > New Pull Request**.

|  Image Illustration  |
|-|
|<img width="492" height="260" alt="Image" src="https://github.com/user-attachments/assets/3901f83d-3b2a-4d30-abfa-d1b6193d9d68" />|

3. Change your fork’s base branch to the one provided by us.  

|  Image Illustration  |
|-|
|<img width="694" height="372" alt="Image" src="https://github.com/user-attachments/assets/70b3274c-0347-4971-adb9-b58f01a14839" />|

4. Create the PR. on the pr description:
   - Add `Closes: #<your-issue-number>` to link your PR to the issue; change "<your-issue-number>" to the actual issue number.
   - Write a **clear description**.


## ⚔️ Step 4: Fix the Conflict

1. On the PR page, you’ll see a banner that says **"This branch has conflicts that must be resolved"**.
2. Click the **Resolve Conflicts** button.

|  Image Illustration  |
|-|
|<img width="846" height="411" alt="Image" src="https://github.com/user-attachments/assets/7a2fbbf0-2735-4831-b9aa-e1e6f257a3c4" />|

3. You have two options:
   - **Edit in place** using GitHub’s UI
   - **Click "View CLI instructions"** to follow steps locally

> 🖥️ CLI version coming soon — contribute to it if you’re familiar!


## ✅ Step 5: Finalise Your Fix

1. After resolving the conflict:
   - Click **"Mark as Resolved"**.
   - Then **"Commit Merge"**.

|  Image Illustration  |
|-|
|<img width="469" height="264" alt="Image" src="https://github.com/user-attachments/assets/52147ebd-bb77-4e6d-9644-e712b55123a4" />|

2. You'll now see that **your PR can be merged**.
3. Assign any team member or maintainer for review.

|  Image Illustration  |
|-|
|<img width="406" height="348" alt="Image" src="https://github.com/user-attachments/assets/03d14417-808d-4a23-aaa1-26c5d12dc9b1" />|

---

## 🎉 You're Done!

Congrats on resolving your first merge conflict! 🚀

That was a real contributor skill — well done! You just:
- Followed a GitHub issue template
- Navigated forks, branches, and PRs
- Fixed a conflict
- Used GitHub’s merge UI

> ✨ **Tip**: Try other issues, such as translations, fixes, or CLI contributions! <br>
> Give this repo a star if you found the task useful

## 📢 Help Improve This Guide

Think you can write a version of this guide for local Git CLI users? Check out this [ISSUE](https://github.com/rezzcode/zoea-opensource/issues/16)

---

Happy contributing 💙


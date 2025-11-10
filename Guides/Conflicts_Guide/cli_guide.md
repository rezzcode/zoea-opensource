# How to Resolve Merge Conflicts via Terminal (CLI Guide)

This guide goes along with the GitHub web UI guide. Here, we will focus on fixing merge conflicts using Git in your terminal. Here's a step-by-step guide so beginners can follow easily. 

### Step 1: Clone the Repository

1. First, fork the repository on GitHub. Then clone your fork locally with this command:

```bash
git clone https://github.com/<your-username>/zoea-opensource.git
```

*Replace `<your-username>` with your GitHub username.*

2. Move into the project directory:
```bash
cd zoea-opensource
```

### Step 2: Checkout the Conflict Branch

1. Fetch all remote branches with the following command:

```bash
git fetch origin
```

2. Then switch to the branch assigned to you.
```bash
git checkout <conflict-branch-name>
```
*Replace `<conflict-branch-name>` with the branch provided in the issue comment*
### Step 3: Merge Main into Your Branch

Merge changes from the main branch into your conflict branch:

```bash
git merge main
```
    
*Git will notify you if there are conflicts*
    
### Step 4: Identify Conflicts

Check which files have conflicts and view what exactly is conflicting with:
    
```bash
git status
```

To see the exact differences, use:
```bash
git diff
```

### Step 5: Resolve Conflicts

Open the conflicted file in your text editor. You'll see markers like this:

```bash
<<<<<<< HEAD
your current branch changes
=======
incoming branch changes
>>>>>>> conflict-branch
```
*Decide which changes to keep, remove the conflict markers, and save the file.*

### Step 6: Stage and Commit Changes

After resolving conflicts, stage the file:
```bash
git add example.md
```
Commit the resolution:
```bash
git commit -m "Resolved merge conflict in example.md"
```

### Step 7: Push Your Changes

Push the updated branch to your fork:
```bash
git push origin <conflict-branch-name>
```

### Step 8: Open a Pull Request

1. Go to your fork on GitHub → Compare & Pull Request

2. Reference the original issue in the PR description:
```bash
Closes #<issue-number>
```
3. Submit the PR. Maintainers will review your changes and provide feedback if needed.

## Tips: 
* Conflicts are normal; everyone faces them. 
* If the terminal seems hard to use, try GitHub Desktop or the VS Code Git extension. 
* Always check your changes with git status and git diff before you commit. 
* Keep your commit messages clear and descriptive.
# Updating Fork Repository Branches 
## Overview
When you create a fork from a repository, it may go out of date as the main repository receives updates. Follow the instructions below to: 
* Update your fork with branches from the upstream repository
* Fetch a new branch from the upstream repository
* Create a new branch or update a pre-existing branch 
* Verify that your branches are up to date
## You Will Need
* A local copy of your fork
* The name of the branch you want to fetch
* Git (ver. 2.56.1)
## Updating Your Fork 
1. Open a terminal in your fork's folder.
2. To find your fork, enter:
        
        git remote -v
        
3. If your fork is listed as `origin`, add the repository as `upstream`:

        git remote add upstream https://github.com/rezzcode/zoea-opensource.git
        
4. Now that your source repository is `upstream`, enter the command below to download the most recent branch information:

        git fetch upstream
        
5. To see which branches you can use, enter:

        git branch -r
        
You will see a list of available branches. If you do not see any new branches, double-check that you are working in the correct terminal and that the base repository is listed as `upstream`, then follow the instructions above again. 

## Adding a New Branch 
1. If you do not have a local copy of the task branch you want to use, enter the command below to create one:

        git switch --track -c <branch-name> upstream/<branch-name>
   
2. To send the branch to your fork, enter:

        git push -u origin <branch-name>
   
3. Enter the command below to verify that the branch is on your computer:

        git branch

The new branch should appear in your terminal after running `git branch` or in your fork's branch selector. If it does not appear, review your commands to make sure the branch names are spelled correctly.

## Updating an Existing Branch
1. If you have an existing branch you want to update, enter the command below to switch to it:

        git fetch upstream
        git switch <branch-name>

2. To bring upstream changes into your branch, enter:

        git merge --ff-only upstream/<branch-name>
   
3.  Once your branch is updated, enter the command below to push the changed branch to your fork:

        git push origin <branch-name>

Your branch should now be up to date. If it has not changed, do not force it. Forcing a push to origin may overwrite the remote branch and create conflicts for others working in the same repository. 

## Checking Your Branches
1. To pull the latest branches from both the upstream repository and your origin fork, enter:

        git fetch upstream
        git fetch origin
        
2. Enter the command below to get a list of the available branches from both repositories:

        git branch -r

3. Check the lists and make sure they both match. *You should see two lists of branches. One formatted as origin/<branch-name> and the other as upstream/<branch-name>. All branch names should match.*
4. OPTIONAL: If you want to double-check a pair of branches, enter:

        git rev-parse upstream/<branch-name>
        git rev-parse origin/<branch-name>

After you run `git rev-parse`, it will return two character strings. If those strings match, the branches are up to date. If they do not match, one of the branches is outdated. 

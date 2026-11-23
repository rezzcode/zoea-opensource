# Updating Fork Repository Branches 
When you create a fork from a repository, it may go out of date as the main repository receives updates. Follow the instructions below to: 
* Update your fork with branches from the upstream repository
* Fetch a new branch from the upstream repository
* Update a pre-existing branch
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

The new branch should appear on your terminal after running `git branch`or in your fork's branch selector. If it does not appear, review your commands to make sure the branch names are spelled correctly.

## Updating an Existing Branch
1. If you have an existing branch you want to update, enter the command below to switch to it:

        git fetch upstream
        git switch <branch-name>

3. Now that you are working 
4. 

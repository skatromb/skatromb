# About me
- Data engineer on MacBook. Google Cloud, AWS, Python, SQL, dbt, docker. Learning Rust

# Rules
- Talk in simple conversational manner
- Make elegant code
- Simple, readable, concise code
- Avoid premature optimisation or features, no unnecessary abstractions
- Don't run disruptive commands without approval
- Be critical. Flag bad patterns or non-idiomatic code, suggest fixes
- No comments in the code
- Edit files with Edit/Write, not scripts.
- Prove a fix by reverting it and re-running, not by reasoning
- During dev cycles, check the Makefile for lint/test/build commands
- Ask to read the diff before commit, push or merge

# Git
- Start each conversation by checking out a git worktree in a `.claude/worktree/` sub-folder
- Commits: one-line subject, no body, no `feat:`/`fix:` prefix, ≤50 chars. Keep the co-author trailer
- PR/issue bodies: first describe why, then how it is implemented in bullet-points, short as possible. No file-by-file recap, no validation block unless manual checks required. 
- Dependent PRs: drive with `gh stack`, never a hand-set base branch
- No 🤖 attribution footer in PR bodies

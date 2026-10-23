# About me
- Not native English speaker. Keep language concise, simple
- Data engineer on MacBook. Google Cloud, AWS, Python, SQL, dbt, docker, Rust

# Conversation
- Answer concise as possible.
- Narrative strictly sequential, no convoluted sentences. "Here is this. It does that. This is bad because of this. Idea to fix is ... here is the code"
- Summarise work by meaning, not per-file changes
- Write all prose replies in ASD-STE100 STE-flavored mode: @~/.claude/skills/asd-ste100/references/writing-rules.md
- Spawn autonomous agents via `claude ...` in shell when needed, keep sub-agent output out of current thread.

# Coding
- Focus on readability
- Meaningful names, no 1-2 letter names
- Prefer removing code over adding
- No premature optimisation/features, no unnecessary abstractions
- Flag bad patterns, non-idiomatic code
- Edit files with Edit/Write, not scripts.
- No disruptive commands without approval
- Dev cycles: check Makefile for lint/test/build commands
- Never silence lint without asking user
- Prove fix by reverting + re-running, not reasoning
- When proposing changes, provide links including worktree to the files you refer to
- No code comments

# Git
- Each branch in separate git worktree at `<repo>/<branch>`, sibling of default-branch clone: `git -C <repo>/<default-branch> worktree add ../<branch> -b <branch>`.
  - After PR merged: cleanup worktree + branch, pull `main`.
- Ask user to review the code before commit, push, merge
- Commits: one-line subject, no body, no `feat:`/`fix:` prefix, ≤50 chars.
- PR/issue bodies: think of draft PR body, then make it SUPER concise. No file-by-file recap, no validation block unless manual checks required.
- Dependent PRs: use `gh stack`, never hand-set base branch
- No 🤖 AI footer in PR bodies

# Docs
- Be concrete: name exact file, show snippet, don't describe location
- One-liners where possible
- Reasoning/internals stay in source; link to it
- Cut what reader doesn't need: stats, mappings, edge cases
- One line per paragraph, no hard wraps
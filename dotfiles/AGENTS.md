# About me
- Not native English speaker. Keep language concise, simple
- Data engineer on MacBook. Google Cloud, AWS, Python, SQL, dbt, docker, Rust

# Conversation
- Only answer my questions, never do a job for me (write code, resolve incidents, query API)
- Общаемся по свойски, на "ты"
- Answer maximum in 3 sentences.
- Narrative strictly sequential, no convoluted sentences. "Here is this. It does that. This is bad because of this. Idea to fix is ... here is the code"
- Summarise work by meaning, not per-file changes
- When run in Zed ACP, spawn autonomous agents via `claude ...` to keep sub-agent output out of current thread
- If you're unsure  why I ask to change or redo something, clarify my intention. When clarified, ask where should we store this knowledge: AGENTS.md or project-specific file
- Don't send message on my behalf without explicit approval

# Coding
- Sort entities in code by "stepdown rule" from the "Clean code"
- Focus on readability
- Meaningful names, no 1-2 letter names
- Prefer removing code over adding
- No premature optimisation/features, no unnecessary abstractions
- Flag bad patterns, non-idiomatic code
- Edit files with Edit/Write, not scripts
- No disruptive commands without approval
- Dev cycles: check Makefile for lint/test/build commands
- Never silence lint without asking user
- Prove fix by reverting + re-running, not reasoning
- Assess fixes by coherence: when it decreases number of edge cases — it's good, if it increases — bad
- When proposing changes, provide links including worktree to the files you refer to
- No code comments

# Git
- Each branch in separate git worktree at `<repo>/<branch>`, sibling of default-branch clone: `git -C <repo>/<default-branch> worktree add ../<branch> -b <branch>`
- Merge by squashing
- Before publishing PR, organize commits so that each contains sensible part of work
- After PR merged: cleanup worktree + branch, pull `main`
- Ask user to review the code before commit, push, merge
- Commits: one-line subject, no body, no `feat:`/`fix:` prefix, ≤50 chars
- PR/issue bodies: think of draft PR body, then make it SUPER concise. No file-by-file recap, just what's done and why done one-liners
- After PR is created or modified, run a `ponytail:ponytail-review` subagent on it
- Dependent PRs: use `gh stack`, never hand-set base branch
- No 🤖 AI footer in PR bodies

# Docs
- Be concrete: name exact file, show snippet, don't describe location
- One-liners where possible
- Reasoning/internals stay in source; link to it
- Cut what reader doesn't need: stats, mappings, edge cases
- One line per paragraph, no hard wraps
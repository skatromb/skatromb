# About me
- Data engineer on MacBook. Google Cloud, AWS, Python, SQL, dbt, docker. Learning Rust

# Rules
- Talk in simple conversational manner
- Summarise work done by meaning, don't go through each file's changes
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

# Writing docs
- Lead with what the reader runs, then explain only what the commands don't show
- Several commands or options: one code block, a short comment per line, no prose repeating them
- Comments and sentences in the reader's words, about the outcome: "apply without asking", not "take the preset's value on every conflict"
- Be concrete: name the exact file and show the snippet instead of describing where it goes
- One line when one line does it. A section can be a single sentence
- Leave reasoning and internals in the source that holds them, and link to it
- Cut what the reader doesn't need to use the thing: stats, mappings, edge cases
- One line per paragraph, no hard wraps

# Git
- Start each conversation by checking out a git worktree in `<repo>/<branch>`, a sibling of the default-branch worktree over a bare `<repo>/.bare`
- Commits: one-line subject, no body, no `feat:`/`fix:` prefix, ≤50 chars. Keep the co-author trailer
- PR/issue bodies: first describe why, then how it is implemented in bullet-points, SUPER concise. No file-by-file recap, no validation block unless manual checks required. 
- Dependent PRs: drive with `gh stack`, never a hand-set base branch
- No 🤖 attribution footer in PR bodies

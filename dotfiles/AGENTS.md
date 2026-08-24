# About me
- Data engineer on MacBook. Google Cloud, AWS, Python, SQL, dbt, docker. Learning Rust

# When making changes
- Make code beautiful
- Simple, readable, concise code
- No premature optimisation or features
- One function at a time, ask feedback
- No disruptive commands without approval
- Stick to most concise solution possible
- Be critical. Flag bad patterns or non-idiomatic code, suggest fixes
- Comments only when absolutely necessary. One line, ≤100 chars — `///`, docstrings and inline `//` alike
- Edit files with Edit/Write, not scripts. Keep Bash calls short — a screen-sized blob jams the approval prompt
- Prove a fix by reverting it and re-running, not by reasoning
- During dev cycles, check the Makefile for lint/test/build commands

# Git
- Finish, run the gate, then stop. I read the diff before you commit, push or merge
- Commits: one-line subject, no body, no `feat:`/`fix:` prefix, ≤50 chars. Keep the co-author trailer
- Dependent PRs: drive with `gh stack`, never a hand-set base branch

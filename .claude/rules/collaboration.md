# PM & Team Collaboration Rules

## Communication Style
- Be concise but complete
- Use bullet points for lists
- Include specific file:line references
- Quote relevant code snippets when discussing changes

## Working with PMs/Stakeholders

### When Asking Questions
- Group related questions together
- Provide context for why you need the answer
- Suggest default options: "Should we use X? (default: Y)"
- Tag the issue author: @username
- If blocked, clearly state: **BLOCKED: need answer to proceed**

### When Giving Updates
- Start with status: ✅ Done / 🚧 In Progress / ❌ Blocked / ❓ Need Input
- Summarize what changed in 1-2 sentences
- Link to relevant PR/commits
- List any remaining work

### When Requesting Review
- Highlight what to focus on
- Note any areas of uncertainty
- Call out breaking changes or risks
- Suggest testing steps

## Feedback Loop

### Responding to Comments
- Acknowledge feedback: "Got it, will..."
- If disagreeing, explain why with evidence
- If clarifying, quote the original comment
- Update the plan if requirements changed

### Iterating on Plans
- When plan feedback arrives, produce "PLAN v2", "PLAN v3", etc.
- Explicitly note what changed from previous version
- If major changes, re-ask verification criteria

## Status Signals (use in comments)

```
🚀 Starting implementation...
✅ Implementation complete - PR ready for review
❌ Build failed - investigating
❓ Need clarification on: <topic>
🔄 Updating based on feedback...
⏳ Long-running task - estimated X minutes
```

## Escalation
- If stuck for 3+ attempts on same issue: ask for help
- If requirements seem contradictory: surface the conflict
- If scope creep detected: ask "Is this in scope for this issue?"

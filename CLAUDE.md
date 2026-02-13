
## Backlog Discipline

**Every bug fix, feature, and release MUST have a corresponding backlog task.**

This includes:
- **Bug fixes** — Create a task (status: Done if already fixed) with root cause, fix description, and affected files
- **Releases** — Tag releases should reference the backlog tasks included
- **Investigations** — Even exploratory work that produces findings gets a task
- **Infrastructure changes** — CI, build system, deployment changes

If the work is already done, create the task retroactively with status `Done`, check all ACs, and write a final summary. No exceptions — this is how we maintain an audit trail.

<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->

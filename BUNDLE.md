# Errday AI Developer Guidelines

## Release Notes & Deployment Publishing

When generating release notes or documenting changes for deployment in this project, you must rigidly adhere to the following conventions:

### 1. File Naming and Location Structure
- All release notes must live strictly in the `docs/releases/` directory.
- The file name format must be exact: `v[VERSION_NUMBER].md` (e.g., `v1.2.7.md`). Do NOT use underscores, spaces, or suffixes like "Release_Notes".

### 2. Document Title Convention
- The primary markdown header (H1) inside the file must always be: `# Release Notes: v[VERSION_NUMBER]`.

### 3. Voice and Tone
- Write exclusively in the **Active Voice** (e.g., "Refactor the calendar view", not "The calendar view was refactored").
- Make action verbs punchy and direct. Drop unnecessary pleasantries and omit emojis altogether.

### 4. Value-Driven Structure
- For **every single bullet point or listed change**, you must immediately follow up with the **business/user value** that the change provides, bolded exactly as `**Value:** [explanation]`.
- Do not simply state what code was changed; focus strictly on what problem the change solves for the end-user or the system infrastructure.

### Example Template Format

```markdown
# Release Notes: vX.Y.Z

## Overview
[1-2 sentences in active voice summarizing the goal of the release.]
**Value:** [Summary of what the user gains from this release.]

## Fixes & Improvements

### [Component Name]
- [Action verb] [what was changed].
  **Value:** [Why the user/business cares].
```

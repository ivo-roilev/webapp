## Context

The UserInfoPage component currently renders a page heading with the text "User Info". This is a generic label that doesn't convey the friendly, personalized nature of the greeting content displayed below it. Changing this to "Greetings!" will improve the user experience by better aligning the heading with the greeting message format.

## Goals / Non-Goals

**Goals:**
- Update the h1 heading in UserInfoPage.tsx from "User Info" to "Greetings!"
- Improve user experience by using more engaging, friendly language
- Ensure change is non-breaking and requires no refactoring

**Non-Goals:**
- Modifying other parts of the UserInfoPage component
- Creating new components or changing component structure
- Updating styling or CSS

## Decisions

### Decision: Simple String Replacement
**Chosen:** Replace the hardcoded string "User Info" with "Greetings!" in the existing h1 element.

**Rationale:** This is a single-line text change. No refactoring, conditional logic, or component restructuring is needed. The change is localized to one location in the codebase.

**Alternatives Considered:**
- Externalize the string to a constant: Rejected (unnecessary complexity for a one-time string)
- Create a configuration file for UI labels: Rejected (overkill for a single label change)
- Parameterize the heading: Rejected (not needed - heading is static)

## Risks / Trade-offs

[Risk] User familiarity: Users may initially expect "User Info" terminology
→ Mitigation: The change is purely cosmetic and doesn't affect functionality. Users will quickly adapt to the new heading.

[Risk] Search/accessibility: Users searching for "User Info" won't find "Greetings!"
→ Mitigation: Low impact since this is internal page text and users access via login flow, not search.

## Migration Plan

**Deployment:**
1. Update the h1 text in `src/web/src/pages/UserInfoPage.tsx` from "User Info" to "Greetings!"
2. No database migrations required
3. No API changes required
4. No build or deployment changes needed

**Rollback:**
Simple one-line revert if needed - revert the text back to "User Info"


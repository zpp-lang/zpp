<div align="center">
<h1>rfcs</h1>
RFCs (request for comments) for Z++ Evolution
</div>

## Overview
Whenever Z++ changes its syntax or semantics, even the standard library (or other libraries
we provide), we have to consider the implications of the changes, especially for backwards compatibility.

Whenever new syntax is introduced, we need to ask;
- Is it backwards compatible?
  - Backwards compatibility in this context means compatibility with ZSharp scripts.
- Is it easy for humans and machines to parse?
- Does it have any challenges for LSP or editor integration?

For changes in semantics, we need to ask;
- Is its behavior backwards compatible to some degree?
- Is its behavior easy to understand?
- Can it be implemented without any performance implications?
- Is it compatible with type checking?

For new standard library functions, we need to ask;
- Is the new functionality going to be useful in existing code?
- Does the implementation have any performance benefits that can't be otherwise gotten in user code?
- Are there already other libraries that user code could link to instead?
  - This is one of the reasons why we will be rewriting the `ZS` library in Z++, as there is
  no reason to include functions with niche use cases in the standard library.
- Is the behavior easily understandable and documented?

Reversing these decisions could very well be impossible, as backwards compatibility is our main goal. Thus,
we require all changes to Z++ to go through an RFC process.

## Process
To open an RFC, a pull request must be opened which creates a new markdown file in the `texts/` folder. The RFCs
should follow the template `TEMPLATE.md` and should have a file name that is a short human-readable description,
e.g. `switch-statements.md`.

We will review the RFC within a week and determine if it aligns with our goals for evolving Z++ during that
time frame. Your pull request will not get merged until it has been implemented. It could take weeks or months
depending on what's going on at the time.

## Implementation
When an RFC is merged, a feature *can* (and probably will) be implemented; however there's no timeline
for an implementation. Once the RFC has been implemented, the document will be updated to have a status
of `Implemented feature flag`, which signifies that we are incubating the feature or have provided a way for
users to use a non-backwards compatible feature. 

### Feature flags
Feature flags tell the compiler that you are okay with not wanting to ensure perfect backwards compatibility
with ZSharp, allowing you to use some innovative features not found in ZSharp. Every RFC is turned into a
feature flag, however not every RFC stays as one.

If the feature is backwards compatible, we will have a feature flag for it until it has finished incubation
and is ready to be included with the compiler by default.